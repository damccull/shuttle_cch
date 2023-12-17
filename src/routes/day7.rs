use std::collections::HashMap;

use actix_web::{get, HttpRequest, HttpResponse, Responder};
use anyhow::Context;
use base64::{engine::general_purpose, Engine as _};
use serde::Serialize;

use crate::CodehuntError;

#[tracing::instrument]
#[get("/7/decode")]
pub async fn decode(request: HttpRequest) -> Result<HttpResponse, CodehuntError> {
    let r = get_recipe_from_header(request).context("Error in recipe cookie")?;
    tracing::debug!("Recipe: {:?}", &r);

    Ok(HttpResponse::Ok().json(r))
}

#[tracing::instrument]
#[get("/7/bake")]
pub async fn bake(request: HttpRequest) -> impl Responder {
    let r = match get_recipe_from_header(request) {
        Ok(r) => r,
        Err(e) => {
            tracing::debug!("Error in recipe cookie: {}", e);
            return HttpResponse::BadRequest().finish();
        }
    };
    tracing::debug!("Recipe: {:?}", &r);

    let bakery = match split_recipe_from_pantry(r) {
        Ok(b) => b,
        Err(e) => {
            tracing::debug!("Error creating bakery: {}", e);
            return HttpResponse::BadRequest().finish();
        }
    };
    tracing::debug!("Bakery created successfully: {:#?}", &bakery);

    let result = calculate_cookies(bakery);
    tracing::debug!("Cookies and remaining pantry: {:#?}", &result);
    HttpResponse::Ok().json(result)
}

#[derive(Debug)]
struct Bakery {
    recipe: HashMap<String, u64>,
    pantry: HashMap<String, u64>,
}

impl Default for Bakery {
    fn default() -> Self {
        Bakery {
            recipe: HashMap::new(),
            pantry: HashMap::new(),
        }
    }
}

#[tracing::instrument]
fn split_recipe_from_pantry(input: serde_json::Value) -> Result<Bakery, CodehuntError> {
    let mut bakery = Bakery::default();

    let recipe = input
        .get("recipe")
        .context("Unable to find recipe in the input")?
        .to_owned();

    let recipe = recipe
        .as_object()
        .context("Unable to get recipe as object")?;

    for (key, value) in recipe.iter() {
        bakery
            .recipe
            .insert(key.clone(), value.as_u64().unwrap_or(0));
    }

    let pantry = input
        .get("pantry")
        .context("Unable to find pantry in input")?
        .to_owned();

    let pantry = pantry
        .as_object()
        .context("Unable to get recipe as object")?;

    for (key, value) in pantry.iter() {
        bakery
            .pantry
            .insert(key.clone(), value.as_u64().unwrap_or(0));
    }

    Ok(bakery)
}

#[derive(Debug, Serialize)]
struct BakeReply {
    cookies: u64,
    pantry: HashMap<String, u64>,
}

#[tracing::instrument]
fn calculate_cookies(bakery: Bakery) -> BakeReply {
    let mut remaining_pantry = HashMap::<String, u64>::new();
    let mut max_cookies_by_ingredient = Vec::<u64>::new();

    for (ingredient, &recipe_amount) in bakery.recipe.iter() {
        // Loop gets the amount the recipe needs

        // Next, get the amount in the pantry
        let pantry_amount = if recipe_amount == 0 {
            tracing::debug!("Setting pantry entry for {} to 0", ingredient.to_string());
            remaining_pantry.insert(ingredient.to_string(), 0);
            0
        } else {
            let Some(&x) = bakery.pantry.get(ingredient) else {
                // None of this ingredient. Push a zero to the counter collector
                max_cookies_by_ingredient.push(0);
                let reply = BakeReply {
                    cookies: 0,
                    pantry: bakery.pantry,
                };
                tracing::debug!("BakeReply: {:#?}", &reply);
                return reply;
            };
            x
        };

        // Elves are dumb so we have to ensure they're not asking for 0 ingredient
        if recipe_amount != 0 && recipe_amount > pantry_amount {
            // Not enough ingredient. Push 0 to cookie counter collection
            max_cookies_by_ingredient.push(0);
            let reply = BakeReply {
                cookies: 0,
                pantry: bakery.pantry,
            };
            tracing::debug!("BakeReply: {:#?}", &reply);
            return reply;
        }

        let cookies_can_bake = if pantry_amount == 0 || recipe_amount == 0 {
            0
        } else {
            pantry_amount / recipe_amount
        };
        tracing::trace!(
            "Ingredient: {}\
        \nNumber of cookies this ingredient can bake: {}",
            &ingredient,
            &cookies_can_bake,
        );
        max_cookies_by_ingredient.push(cookies_can_bake);
    }
    let cookies_can_be_baked = *max_cookies_by_ingredient.iter().min().unwrap_or(&0);
    tracing::debug!("Cookies that can be baked: {}", &cookies_can_be_baked);
    for (ingredient, &recipe_amount) in bakery.recipe.iter() {
        // Loop gets the amount the recipe needs
        let pantry_amount = bakery.pantry[ingredient];

        remaining_pantry.insert(
            ingredient.clone(),
            pantry_amount - (recipe_amount * cookies_can_be_baked),
        );
    }

    let reply = BakeReply {
        cookies: cookies_can_be_baked,
        pantry: remaining_pantry,
    };

    tracing::debug!("BakeReply: {:#?}", &reply);
    reply
}

#[tracing::instrument]
fn get_recipe_from_header(request: HttpRequest) -> Result<serde_json::Value, CodehuntError> {
    let recipe_cookie = request
        .cookie("recipe")
        .context("No cookie recipe in cookie jar")?;

    let recipe = recipe_cookie.to_string();
    tracing::trace!("ToString: {:#?}", &recipe);

    let (_, recipe) = recipe
        .split_once("=")
        .context("Badly formed recipe cookie")?;
    tracing::trace!("Split: {:#?}", &recipe);

    let recipe = general_purpose::STANDARD
        .decode(recipe)
        .context("Unable to base64 decode the cookie.")?;
    tracing::trace!("base64 decode: {:#?}", &recipe);

    let recipe =
        serde_json::from_slice::<serde_json::Value>(&recipe).context("Unable to parse to JSON")?;
    tracing::trace!("Json: {:#?}", &recipe);

    Ok(recipe)
}

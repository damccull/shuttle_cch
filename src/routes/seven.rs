use std::{collections::HashMap, str::FromStr};

use actix_web::{get, HttpRequest, HttpResponse, Responder};
use base64::{engine::general_purpose, Engine as _};
use serde::Serialize;

#[tracing::instrument]
#[get("/7/decode")]
pub async fn decode(request: HttpRequest) -> impl Responder {
    let r = match get_recipe_from_header(request) {
        Ok(r) => r,
        Err(e) => {
            tracing::debug!("Error in recipe cookie: {}", e);
            return HttpResponse::BadRequest().finish();
        }
    };
    tracing::debug!("Recipe: {:?}", &r);
    HttpResponse::Ok().json(r)
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
fn split_recipe_from_pantry(input: serde_json::Value) -> Result<Bakery, RecipeParseError> {
    let mut bakery = Bakery::default();

    let recipe = input
        .get("recipe")
        .ok_or_else(|| anyhow::anyhow!("Unable to find recipe in input"))?
        .to_owned();

    let recipe = recipe
        .as_object()
        .ok_or_else(|| anyhow::anyhow!("Unable to get recipe as object"))?;

    for (key, value) in recipe.iter() {
        bakery
            .recipe
            .insert(key.clone(), value.as_u64().unwrap_or(0));
    }

    let pantry = input
        .get("pantry")
        .ok_or_else(|| anyhow::anyhow!("Unable to find pantry in input"))?
        .to_owned();

    let pantry = pantry
        .as_object()
        .ok_or_else(|| anyhow::anyhow!("Unable to get recipe as object"))?;

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
        let Some(&pantry_amount) = bakery.pantry.get(ingredient) else {
            // None of this ingredient. Push a zero to the counter collector
            max_cookies_by_ingredient.push(0);
            return BakeReply {
                cookies: 0,
                pantry: bakery.pantry,
            };
        };

        if recipe_amount > pantry_amount {
            // Not enough ingredient. Push 0 to cookie counter collection
            max_cookies_by_ingredient.push(0);
            return BakeReply {
                cookies: 0,
                pantry: bakery.pantry,
            };
        }

        let cookies_can_bake = pantry_amount / recipe_amount;
        tracing::trace!(
            "Ingredient: {}\
        \nNumber of cookies this ingredient can bake: {}",
            &ingredient,
            &cookies_can_bake,
        );
        max_cookies_by_ingredient.push(cookies_can_bake);
    }
    let cookies_can_be_baked = *max_cookies_by_ingredient.iter().min().unwrap_or(&0);

    for (ingredient, &recipe_amount) in bakery.recipe.iter() {
        // Loop gets the amount the recipe needs
        let pantry_amount = bakery.pantry[ingredient];

        remaining_pantry.insert(
            ingredient.clone(),
            pantry_amount - (recipe_amount * cookies_can_be_baked),
        );
    }

    BakeReply {
        cookies: cookies_can_be_baked,
        pantry: remaining_pantry,
    }
}

#[tracing::instrument]
fn get_recipe_from_header(request: HttpRequest) -> Result<serde_json::Value, RecipeParseError> {
    let Some(recipe_cookie) = request.cookie("recipe") else {
        return Err(RecipeParseError::UnexpectedError(anyhow::anyhow!(
            "No cookie recipe in cookies"
        )));
    };

    let recipe = recipe_cookie.to_string();
    tracing::trace!("ToString: {:#?}", &recipe);

    let Some((_, recipe)) = recipe.split_once("=") else {
        return Err(RecipeParseError::UnexpectedError(anyhow::anyhow!(
            "Badly formed recipe cookie"
        )));
    };
    tracing::trace!("Split: {:#?}", &recipe);

    let recipe = match general_purpose::STANDARD.decode(recipe) {
        Ok(r) => r,
        Err(e) => {
            return Err(RecipeParseError::UnexpectedError(anyhow::anyhow!(
                "Unable to base64 decode the cookie: {}",
                e
            )));
        }
    };
    tracing::trace!("base64 decode: {:#?}", &recipe);

    let Ok(recipe) = std::str::from_utf8(&recipe) else {
        return Err(RecipeParseError::UnexpectedError(anyhow::anyhow!(
            "Can't convert decoded to string"
        )));
    };
    tracing::trace!("Convert to str: {:#?}", &recipe);

    let recipe = serde_json::Value::from_str(recipe).map_err(|e| {
        RecipeParseError::UnexpectedError(anyhow::anyhow!("Unable to parse to JSON: {}", e))
    });
    tracing::trace!("Json: {:#?}", &recipe);

    recipe
}

#[derive(thiserror::Error)]
enum RecipeParseError {
    #[error(transparent)]
    DecodeError(#[from] base64::DecodeError),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for RecipeParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        crate::error_chain_fmt(self, f)
    }
}

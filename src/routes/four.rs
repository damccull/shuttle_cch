use std::fmt::Display;

use actix_web::{web::Json, HttpResponse, Responder, post};
use serde::{Deserialize, Serialize};

#[tracing::instrument]
#[post("/4/strength")]
pub async fn strength(reindeer: Json<Vec<Reindeer>>) -> impl Responder {
    tracing::debug!("{:#?}", reindeer);
    let result = reindeer.iter().fold(0u32, |acc, deer| acc + deer.strength);
    HttpResponse::Ok().body(result.to_string())
}

#[tracing::instrument]
#[post("/4/contest")]
pub async fn contest(reindeer: Json<Vec<ReindeerContestEntry>>) -> impl Responder {
    if let Ok(winners) = determine_winners(reindeer.0) {
        return HttpResponse::Ok().json(winners);
    }
    HttpResponse::UnprocessableEntity()
        .body("Probably incorrect json. Ensure you match the deer schema!")
}

#[tracing::instrument]
fn determine_winners(reindeer: Vec<ReindeerContestEntry>) -> Result<ContestWinners, anyhow::Error> {
    let fastest = reindeer
        .iter()
        .max_by(|d1, d2| d1.speed.total_cmp(&d2.speed))
        .ok_or_else(|| anyhow::anyhow!("No fastest deer or a tie"))?;
    let tallest = reindeer
        .iter()
        .max_by_key(|d| d.height)
        .ok_or_else(|| anyhow::anyhow!("No tallest deer or a tie"))?;
    let magician = reindeer
        .iter()
        .max_by_key(|d| d.snow_magic_power)
        .ok_or_else(|| anyhow::anyhow!("No best magician or a tie"))?;
    let consumer = reindeer
        .iter()
        .max_by_key(|d| d.candies_eaten_yesterday)
        .ok_or_else(|| anyhow::anyhow!("No most gluttonous deer or a tie"))?;
    Ok(ContestWinners {
        fastest: format!(
            "Speeding past the finish line with a strength of {} is {}",
            fastest.strength, fastest
        ),
        tallest: format!(
            "{} is standing tall with his {} cm wide antlers",
            tallest, tallest.antler_width
        ),
        magician: format!(
            "{} could blast you away with a snow magic power of {}",
            magician, magician.snow_magic_power
        ),
        consumer: format!("{} ate lots of candies, but also some grass", consumer),
    })
}

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize)]
pub struct Reindeer {
    name: String,
    strength: u32,
}

#[allow(dead_code)]
#[derive(Clone, Debug, Deserialize)]
pub struct ReindeerContestEntry {
    name: String,
    strength: u32,
    speed: f32,
    height: u32,
    antler_width: u32,
    snow_magic_power: u32,
    favorite_food: String,
    #[serde(rename = "cAnD13s_3ATeN-yesT3rdAy")]
    candies_eaten_yesterday: u32,
}

impl Display for ReindeerContestEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct ContestWinners {
    fastest: String,
    tallest: String,
    magician: String,
    consumer: String,
}

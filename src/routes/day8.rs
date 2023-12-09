use actix_web::{get, web::Path, HttpResponse};
use anyhow::Context;

const GRAVITATIONAL_ACCELERATION: f32 = 9.825;

#[tracing::instrument]
#[get("/8/weight/{id}")]
pub async fn pokeweight(id: Path<f32>) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let weight_hectograms = PokeWeightGraphQlQuery::new(*id)
        .fetch_weight_from_pokemon_api()
        .await?;
    tracing::trace!("Pokemon weight from API: {}", &weight_hectograms);
    let weight = weight_hectograms as f32 / 10f32;
    Ok(HttpResponse::Ok().body(weight.to_string()))
}

#[tracing::instrument]
#[get("/8/drop/{id}")]
pub async fn pokedrop(id: Path<f32>) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    let weight_hectograms = PokeWeightGraphQlQuery::new(*id)
        .fetch_weight_from_pokemon_api()
        .await?;
    tracing::trace!("Pokemon weight from API: {}", &weight_hectograms);
    let speed = (2f32 * GRAVITATIONAL_ACCELERATION * 10f32).sqrt();
    let momentum = ((weight_hectograms / 10f64) as f32) * speed;
    tracing::trace!("Pokemon momentum: {}", &momentum);
    Ok(HttpResponse::Ok().body(momentum.to_string()))
}

#[derive(Debug)]
struct PokeWeightGraphQlQuery {
    id: f32,
}
impl PokeWeightGraphQlQuery {
    fn new(id: f32) -> Self {
        Self { id }
    }

    fn to_graphql(&self) -> String {
        format!(
            r#"{{"query": "query samplePokeAPIquery {{pokemon:pokemon_v2_pokemon(where: {{id: {{_eq: {}}}}}) {{id,name,weight,}}}}","variables": {{}}}}"#,
            self.id
        )
    }

    #[tracing::instrument]
    async fn fetch_weight_from_pokemon_api(&self) -> Result<f64, anyhow::Error> {
        let address = "https://beta.pokeapi.co/graphql/v1beta";
        let body = PokeWeightGraphQlQuery::new(self.id);
        tracing::debug!("PokeAPI query: {:#?}", &body.to_graphql());
        let pokeapi_result = reqwest::Client::new()
            .post(address)
            .header("Content-Type", "application/json")
            .body(body.to_graphql())
            .send()
            .await
            .context("Request to pokemon api failed")?
            .json::<serde_json::Value>()
            .await
            .context("Unable to parse json")?;
        tracing::debug!("PokeAPI result: {:#?}", &pokeapi_result);

        let weight = pokeapi_result
            .get("data")
            .ok_or_else(|| anyhow::anyhow!("Unable to find 'data' object in json"))?
            .get("pokemon")
            .ok_or_else(|| anyhow::anyhow!("Unable to find 'pokemon' array"))?
            .as_array()
            .ok_or_else(|| anyhow::anyhow!("Unable to convert to Vec"))?
            .first()
            .ok_or_else(|| anyhow::anyhow!("Couldn't get the first pokemon from the array"))?
            .get("weight")
            .ok_or_else(|| anyhow::anyhow!("Couldn't get pokemon's weight"))?
            .as_f64()
            .ok_or_else(|| anyhow::anyhow!("Couldn't convert to f64"))?;

        Ok(weight)
    }
}

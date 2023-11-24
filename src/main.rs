use actix_web::{get, web::ServiceConfig, HttpResponse, Responder};
use cch::telemetry;
use shuttle_actix_web::ShuttleActixWeb;

#[get("/")]
#[tracing::instrument]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[get("/health_check")]
#[tracing::instrument]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("Healthy. Enough said.")
}

#[shuttle_runtime::main]
#[tracing::instrument]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    // Set up tracing
    let subscriber =
        telemetry::get_subscriber("shuttle-cch".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    tracing::info!("Tracing enabled.");

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world).service(health_check);
    };
    tracing::info!("Services online.");

    Ok(config.into())
}

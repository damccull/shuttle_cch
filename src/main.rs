use actix_web::{get, web::ServiceConfig};
use cch::{telemetry,routes::health_check_get};
use shuttle_actix_web::ShuttleActixWeb;



#[get("/")]
#[tracing::instrument]
async fn hello_world() -> &'static str {
    "Let the Christmas Code Hunt begin!"
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
        cfg.service(hello_world).service(health_check_get);
    };
    tracing::info!("Services online.");

    Ok(config.into())
}

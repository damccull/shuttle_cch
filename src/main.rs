use actix_web::web::ServiceConfig;
use cch::{telemetry,routes::{health_check, home}};
use shuttle_actix_web::ShuttleActixWeb;





#[shuttle_runtime::main]
#[tracing::instrument]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    // Set up tracing
    let subscriber =
        telemetry::get_subscriber("shuttle-cch".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    tracing::info!("Tracing enabled.");

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(home).service(health_check);
    };
    tracing::info!("Services online.");

    Ok(config.into())
}

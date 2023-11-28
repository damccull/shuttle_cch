use actix_web::web::ServiceConfig;
use cch_23::{
    routes::{health_check, home},
    telemetry,
};

#[cfg(feature = "shuttle")]
use shuttle_actix_web::ShuttleActixWeb;

// If the 'shuttle' features is enabled, run this in shuttle's actix_web runner.
#[cfg(feature = "shuttle")]
#[shuttle_runtime::main]
#[tracing::instrument]
async fn shuttle_main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static>
{
    setup_tracing();

    tracing::info!("Tracing enabled.");

    let config = move |cfg: &mut ServiceConfig| {
        configure_routes(cfg);
    };
    tracing::info!("Services online.");

    Ok(config.into())
}

// If the 'shuttle' feature is disabled, run this in actix_web's native runner.
#[cfg(not(feature = "shuttle"))]
#[actix_web::main]
#[tracing::instrument]
async fn main() -> std::io::Result<()> {
    use actix_web::{App, HttpServer};
    setup_tracing();
    tracing::info!("Tracing enabled.");

    let config = move |cfg: &mut ServiceConfig| {
        configure_routes(cfg);
    };
    HttpServer::new(move || App::new().configure(config))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}

fn setup_tracing() {
    // Set up tracing
    let subscriber =
        telemetry::get_subscriber("shuttle-cch".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);
}

fn configure_routes(cfg: &mut ServiceConfig) {
    cfg.service(home).service(health_check);
}

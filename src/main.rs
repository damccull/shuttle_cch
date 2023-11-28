use cch_23::telemetry;

// Crates needed by the shuttle implementation but that must be in root level
#[cfg(feature = "shuttle")]
use {actix_web::web::ServiceConfig, shuttle_actix_web::ShuttleActixWeb};

// If the 'shuttle' features is enabled, run this in shuttle's actix_web runner.
#[cfg(feature = "shuttle")]
#[shuttle_runtime::main]
#[tracing::instrument]
async fn shuttle_main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static>
{
    use cch_23::application;

    setup_tracing();

    tracing::info!("Tracing enabled.");

    let config = move |cfg: &mut ServiceConfig| {
        application::configure_services(cfg);
    };
    tracing::info!("Services online.");

    Ok(config.into())
}

// If the 'shuttle' feature is disabled, run this in actix_web's native runner.
#[cfg(not(feature = "shuttle"))]
#[actix_web::main]
#[tracing::instrument]
async fn main() -> std::io::Result<()> {
    use cch_23::application::Application;

    setup_tracing();
    tracing::info!("Tracing enabled.");

    let app = Application::build("127.0.0.1".to_string(), 8000).await?;

    app.run_until_stopped().await?;
    Ok(())
}

fn setup_tracing() {
    // Set up tracing
    let subscriber =
        telemetry::get_subscriber("shuttle-cch".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);
}

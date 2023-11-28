use actix_web::web::ServiceConfig;
use cch_23::{
    routes::{health_check, home},
    telemetry,
};

#[cfg(shuttle)]
use shuttle_actix_web::ShuttleActixWeb;

/// [`main`] return type when `shuttle` feature is active
#[cfg(shuttle)]
type MainReturn = ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static>;

/// [`main`] return type when `shuttle` feature is disabled
#[cfg(not(feature="shuttle"))]
type MainReturn = Result<(), ()>;

// If the 'shuttle' features is enabled, run this in shuttle's actix_web runner,
// otherwise use tokio's runner
#[cfg_attr(feature="shuttle", shuttle_runtime::main)]
#[cfg_attr(not(feature="shuttle"), tokio::main)]
#[tracing::instrument]
async fn main() -> MainReturn {
    // Set up tracing
    let subscriber =
        telemetry::get_subscriber("shuttle-cch".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    tracing::info!("Tracing enabled.");

    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(home).service(health_check);
    };
    tracing::info!("Services online.");

    #[cfg(feature="shuttle")]
    {
        Ok(config.into())
    }
    #[cfg(not(feature="shuttle"))]
    {
        println!("No shuttle");
        Ok(())
    }
}

use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

use actix_web::{
    dev::Server,
    web::{self, ServiceConfig},
    App, HttpServer, middleware,
};

use crate::routes::{health_check, home, neg_one_bonus_return_error};

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(listen_addr: String, listen_port: u16) -> Result<Self, std::io::Error> {
        let address = format!("{}:{}", listen_addr, listen_port);
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(listener)?;
        Ok(Self { port, server })
    }

    pub async fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let config = move |cfg: &mut ServiceConfig| {
        configure_services(cfg);
    };
    let server = HttpServer::new(move || App::new().configure(config))
        .listen(listener)?
        .run();

    Ok(server)
}

pub fn configure_services(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(home)
            .service(health_check)
            .service(neg_one_bonus_return_error)
            .wrap(middleware::NormalizePath::trim())
            .wrap(TracingLogger::default()),
    );
}

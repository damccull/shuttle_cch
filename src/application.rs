use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

use actix_web::{
    dev::Server,
    middleware,
    web::{self, ServiceConfig},
    App, HttpServer,
};

use crate::routes::{five, four, health_check, home, neg_one, one, seven, six};

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(listen_addr: &str, listen_port: u16) -> Result<Self, std::io::Error> {
        let address = format!("{}:{}", listen_addr, listen_port);
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = build_server(listener)?;
        Ok(Self { port, server })
    }

    pub async fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn build_server(listener: TcpListener) -> Result<Server, std::io::Error> {
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
            .service(neg_one::bonus_return_error)
            .service(one::xor_power3)
            .service(four::strength)
            .service(four::contest)
            .service(five::grinch)
            .service(six::elf_on_a_self)
            .service(seven::decode)
            .service(seven::bake)
            .wrap(middleware::NormalizePath::trim())
            .wrap(TracingLogger::default()),
    );
}

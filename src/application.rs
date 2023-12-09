use std::net::TcpListener;
use tracing_actix_web::TracingLogger;

use actix_web::{
    dev::Server,
    middleware,
    web::{self, ServiceConfig},
    App, HttpServer,
};

use crate::routes::{day0, day1, day4, day5, day6, day7, day8, health_check};

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
            .service(health_check)
            .service(day0::home)
            .service(day0::bonus_return_error)
            .service(day1::xor_power3)
            .service(day4::strength)
            .service(day4::contest)
            .service(day5::grinch)
            .service(day6::elf_on_a_self)
            .service(day7::decode)
            .service(day7::bake)
            .service(day8::pokeweight)
            .service(day8::pokedrop)
            .wrap(middleware::NormalizePath::trim())
            .wrap(TracingLogger::default()),
    );
}

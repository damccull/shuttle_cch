use cch_23::{application::Application, telemetry::{get_subscriber, init_subscriber}};
use once_cell::sync::Lazy;

static TRACING: Lazy<()> = Lazy::new(|| {
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(
            "test".into(),
            "cch_23=debug,cch_damccull=debug,info".into(),
            std::io::stdout,
        );
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(
            "test".into(),
            "cch_23=debug,cch_damccull=debug,info".into(),
            std::io::sink,
        );
        init_subscriber(subscriber);
    }
});

pub async fn spawn_app() -> TestApp {
    // Setup tracing for tests
    Lazy::force(&TRACING);

    let port = 0; // Get random OS port
    let address = "127.0.0.1"; // More complex than necessary but allows future ease of expansion

    let app = Application::build(address, port)
        .await
        .expect("Failed to build application");

    let port = app.port().await;
    let address = format!("http://{}:{}", address, port);
    
    tokio::spawn(app.run_until_stopped());

    let test_app = TestApp {
        address: address,
        port,
    };

    test_app
}

#[derive(Debug)]
pub struct TestApp {
    pub address: String,
    pub port: u16,
}

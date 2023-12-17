use std::time::Duration;

use anyhow::Context;

use crate::helpers::spawn_app;

#[tokio::test]
async fn static_asset_returns_correct_file() -> Result<(), anyhow::Error> {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/11/assets/decoration.png", app.address))
        .timeout(Duration::from_secs(2))
        .send()
        .await
        .expect("failed to execute request");

    tracing::debug!("{:#?}", &response);
    // Assert
    assert!(
        response.status().is_success(),
        "Ensure the correct file `decoration.png` is in the static directory"
    );
    let content_type = response
        .headers()
        .get("Content-Type")
        .context("Couldn't get `Content-Type` header")?
        .to_str()
        .context("Couldn't read `Content-Type` header as an ASCII string")?;
    assert_eq!(
        content_type, "image/png",
        "Ensure the correct file `decoration.png` is in the static directory"
    );
    assert_eq!(
        response.content_length().unwrap(),
        787297,
        "Ensure the correct file `decoration.png` is in the static directory"
    );
    Ok(())
}

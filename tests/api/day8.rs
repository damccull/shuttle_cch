use std::time::Duration;

use crate::helpers::spawn_app;

#[tokio::test]
async fn weight_returns_correct_weight_for_pokemon() -> Result<(), Box<dyn std::error::Error>> {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/8/weight/25", app.address))
        .timeout(Duration::from_secs(2))
        .send()
        .await
        .expect("failed to execute request");

    tracing::debug!("{:#?}", &response);
    // Assert
    assert!(response.status().is_success());
    assert_eq!("6", response.text().await?);
    Ok(())
}
#[tokio::test]
async fn drop_returns_correct_momentum_for_pokemon() -> Result<(), Box<dyn std::error::Error>> {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/8/drop/25", app.address))
        .timeout(Duration::from_secs(2))
        .send()
        .await
        .expect("failed to execute request");

    tracing::debug!("{:#?}", &response);
    // Assert
    assert!(response.status().is_success());
    let target = "84.10707461325713".parse::<f32>()?;
    let attempt = response.text().await?.parse::<f32>()?;
    assert!(target + 0.001 >= attempt && target - 0.001 <= attempt);

    Ok(())
}

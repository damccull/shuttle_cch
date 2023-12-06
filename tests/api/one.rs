use crate::helpers::spawn_app;

#[tokio::test]
async fn xor_power3_returns_correct_number() -> Result<(), Box<dyn std::error::Error>> {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/1/1/2/3/4/5/6", app.address))
        .send()
        .await
        .expect("failed to execute request");

    tracing::debug!("{:#?}", &response);

    // Assert
    assert!(response.status().is_success());
    let result = response.text().await?.parse::<u32>()?;
    assert_eq!(result, 343u32);
    Ok(())
}

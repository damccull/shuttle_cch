use cch_23::routes::six::ElfReply;

use crate::helpers::spawn_app;
#[tokio::test]
async fn elves() -> Result<(), Box<dyn std::error::Error>> {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "there is an elf on a shelf on an elf.
      there is also another shelf in Belfast."
        .to_string();

    // Act
    let response = client
        .post(format!("{}/6", app.address))
        .header("Content-Type", "text/plain")
        .body(body)
        .send()
        .await
        .expect("failed to execute request");

    tracing::debug!("{:#?}", &response);
    // Assert

    assert!(response.status().is_success());
    let elves = response.json::<ElfReply>().await?;
    assert_eq!(elves.elf, 5);
    Ok(())
}

#[tokio::test]
async fn elves_and_shelves() -> Result<(), Box<dyn std::error::Error>> {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "there is an elf on a shelf on an elf.
      there is also another shelf in Belfast."
        .to_string();

    // Act
    let response = client
        .post(format!("{}/6", app.address))
        .header("Content-Type", "text/plain")
        .body(body)
        .send()
        .await
        .expect("failed to execute request");

    tracing::debug!("{:#?}", &response);
    // Assert

    let answer_key = serde_json::from_value::<ElfReply>(serde_json::json!({
      "elf": 5,
      "elf on a shelf": 1,
      "shelf with no elf on it": 1,
    }))?;

    assert!(response.status().is_success());
    let elves = response.json::<ElfReply>().await?;
    assert_eq!(elves, answer_key);
    Ok(())
}

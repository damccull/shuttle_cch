use std::str::FromStr;

use crate::helpers::spawn_app;

#[tokio::test]
async fn decode_correctly_returns_the_recipe() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/7/decode", app.address))
        .header(
            "Cookie",
            "recipe=eyJmbG91ciI6MTAwLCJjaG9jb2xhdGUgY2hpcHMiOjIwfQ==",
        )
        .send()
        .await
        .expect("failed to execute request");

    // Assert
    let answer_key = serde_json::json!({
        "flour": 100,
        "chocolate chips":20,
    });
    let result = serde_json::Value::from_str(response.text().await.unwrap().as_str()).unwrap();
    assert_eq!(result, answer_key);
}

#[tokio::test]
async fn bake_correctly_returns_the_number_of_cookies_possible_with_remaining_pantry() {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/7/bake", app.address))
        .header(
            "Cookie",
            "recipe=eyJyZWNpcGUiOnsiZmxvdXIiOjk1LCJzdWdhciI6NTAsImJ1dHRlciI6MzAsImJha2luZyBwb3dkZXIiOjEwLCJjaG9jb2xhdGUgY2hpcHMiOjUwfSwicGFudHJ5Ijp7ImZsb3VyIjozODUsInN1Z2FyIjo1MDcsImJ1dHRlciI6MjEyMiwiYmFraW5nIHBvd2RlciI6ODY1LCJjaG9jb2xhdGUgY2hpcHMiOjQ1N319",
        )
        .send()
        .await
        .expect("failed to execute request");

    // Assert
    let answer_key = serde_json::json!({
      "cookies": 4,
      "pantry": {
        "flour": 5,
        "sugar": 307,
        "butter": 2002,
        "baking powder": 825,
        "chocolate chips": 257
      }
    });
    let result = serde_json::Value::from_str(response.text().await.unwrap().as_str()).unwrap();
    assert_eq!(result, answer_key);
}

#[tokio::test]
async fn bake_correctly_returns_number_of_cookies_possible_with_remaining_pantry_when_random_ingredients_exist(
) {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/7/bake", app.address))
        .header(
            "Cookie",
            "recipe=eyJyZWNpcGUiOnsic2xpbWUiOjl9LCJwYW50cnkiOnsiY29iYmxlc3RvbmUiOjY0LCJzdGljayI6IDR9fQ==",
        )
        .send()
        .await
        .expect("failed to execute request");

    // Assert
    let answer_key = serde_json::json!({
      "cookies": 0,
      "pantry": {
        "cobblestone": 64,
        "stick": 4
      }
    });
    let result = serde_json::Value::from_str(response.text().await.unwrap().as_str()).unwrap();
    assert_eq!(result, answer_key);
}

use cch_23::routes::day4::ContestWinners;

use crate::helpers::spawn_app;

#[tokio::test]
async fn strength_returns_correct_number() -> Result<(), Box<dyn std::error::Error>> {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = serde_json::json!([
      { "name": "Dasher", "strength": 5 },
      { "name": "Dancer", "strength": 6 },
      { "name": "Prancer", "strength": 4 },
      { "name": "Vixen", "strength": 7 }
    ]);
    // Act
    let response = client
        .post(format!("{}/4/strength", app.address))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .expect("failed to execute request");

    tracing::debug!("{:#?}", &response);
    // Assert
    assert!(response.status().is_success());
    assert_eq!("22", response.text().await?);
    Ok(())
}

#[tokio::test]
async fn contest_returns_correct_json() -> Result<(), Box<dyn std::error::Error>> {
    // Arrange
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    tracing::debug!("{:#?}", &app);

    let body = serde_json::json!([
      {
        "name": "Dasher",
        "strength": 5,
        "speed": 50.4,
        "height": 80,
        "antler_width": 36,
        "snow_magic_power": 9001,
        "favorite_food": "hay",
        "cAnD13s_3ATeN-yesT3rdAy": 2
      },
      {
        "name": "Dancer",
        "strength": 6,
        "speed": 48.2,
        "height": 65,
        "antler_width": 37,
        "snow_magic_power": 4004,
        "favorite_food": "grass",
        "cAnD13s_3ATeN-yesT3rdAy": 5
      }
    ]);

    // Act
    let response = client
        .post(format!("{}/4/contest", app.address))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .expect("failed to execute request");

    tracing::debug!("{:#?}", &response);
    // Assert
    let answer_key = serde_json::from_value::<ContestWinners>(serde_json::json!({
      "fastest": "Speeding past the finish line with a strength of 5 is Dasher",
      "tallest": "Dasher is standing tall with his 36 cm wide antlers",
      "magician": "Dasher could blast you away with a snow magic power of 9001",
      "consumer": "Dancer ate lots of candies, but also some grass"
    }))?;
    assert!(response.status().is_success());
    let contest_results = response.json::<ContestWinners>().await?;
    assert_eq!(contest_results, answer_key);
    Ok(())
}

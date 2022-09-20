use crate::helpers::*;
use chrono::prelude::*;


#[tokio::test]
async fn add_asset_returns_200_for_valid_data() {
    // Arrange
    let app = spawn_test_application().await;
    let body = serde_json::json!({
        "asset_id": "06929",
        "name": "RHS-LAP-017",
        "serial_num": "1LLFNV2",
        "brand": "Dell",
        "model": "Lattitude 3190 2-in-1"
    });

    // Act
    let response = app.post_add_asset(&body).await;

    // Assert
    assert_eq!(303, response.status().as_u16());

    let saved = sqlx::query!("SELECT asset_id, name, serial_num, brand, model FROM assets",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved asset.");

    assert_eq!(saved.asset_id, "06929");
    assert_eq!(saved.name, "RHS-LAP-017");
}
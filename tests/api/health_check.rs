use crate::helpers::*;

#[tokio::test]
async fn health_check_works() {
    // Arrange
    let app = spawn_test_application().await;

    // Act
    let response = app.get_health_check().await;

    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
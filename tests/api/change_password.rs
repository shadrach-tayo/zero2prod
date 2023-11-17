use crate::helpers::{assert_is_redirect_to, spawn_app};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[tokio::test]
async fn you_must_be_logged_in_to_see_the_change_password_form() {
    // Arrange
    let app = spawn_app().await;
    // Act
    let response = app.get_change_password().await;
    // Assert
    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn you_must_be_logged_in_to_change_your_password() {
    // Arrange
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();
    // Act
    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": Uuid::new_v4().to_string(),
            "new_password": &new_password,
            "new_password_check": &new_password,
        }))
        .await;
    // Assert
    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn new_password_fields_must_match() {
    // Arrange
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();
    let another_password = Uuid::new_v4().to_string();

    // LOGIN
    app.post_login(&serde_json::json!({
        "username": app.test_user.username,
        "password": app.test_user.password,
    }))
    .await;

    // Try to change password
    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": Uuid::new_v4().to_string(),
            "new_password": &new_password,
            "new_password_check": &another_password,
        }))
        .await;
    // Assert
    assert_is_redirect_to(&response, "/admin/password");

    // Act - Part 3 - Follow the redirect
    let html_page = app.get_change_password_html().await;
    assert!(html_page.contains(
        "<p><i>You entered two different new passwords - \
            the field values must match.</i></p>"
    ));
}

#[tokio::test]
async fn current_password_must_be_valid() {
    // Arrange
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();
    let wrong_password = Uuid::new_v4().to_string();

    // LOGIN
    app.post_login(&serde_json::json!({
        "username": app.test_user.username,
        "password": app.test_user.password,
    }))
    .await;

    // Try to change password
    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": &wrong_password,
            "new_password": &new_password,
            "new_password_check": &new_password,
        }))
        .await;
    // Assert
    assert_is_redirect_to(&response, "/admin/password");

    // Act - Part 3 - Follow the redirect
    let html_page = app.get_change_password_html().await;
    assert!(html_page.contains("<p><i>The current password is incorrect.</i></p>"));
}

#[tokio::test]
async fn invalid_password_is_rejected() {
    let app = spawn_app().await;

    let new_password = "new";

    // LOGIN
    app.post_login(&serde_json::json!({
        "username": app.test_user.username,
        "password": app.test_user.password,
    }))
    .await;

    // Try to change password
    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": app.test_user.password,
            "new_password": &new_password,
            "new_password_check": &new_password,
        }))
        .await;
    // Assert
    #[derive(Deserialize, Serialize)]
    struct ErrorResponse {
        pub error: String,
    }
    let status = response.status().as_u16().clone();
    let error_response = &response.json::<ErrorResponse>().await.unwrap();
    assert_eq!(
        error_response.error,
        "Password should be longer that 4 characters and less than 128 characters.".to_string()
    );
    assert_eq!(status, 400);
}
#[tokio::test]
async fn logout_clears_user_session() {
    let app = spawn_app().await;

    let new_password = "new";

    // LOGIN
    let response = app
        .post_login(&serde_json::json!({
            "username": app.test_user.username,
            "password": app.test_user.password,
        }))
        .await;
    assert_is_redirect_to(&response, "/admin/dashboard");

    let html_page = app.get_admin_dashboard_html().await;
    assert!(html_page.contains(&format!("Welcome {}", app.test_user.username)));

    let response = app.post_logout().await;
    assert_is_redirect_to(&response, "/login");

    let html_page = app.get_login_html().await;
    assert!(html_page.contains(r#"<p><i>You have successfully logged out.</i></p>"#));
    // dbg!(&html_page);

    let response = app.get_admin_dashboard().await;
    assert_is_redirect_to(&response, "/login")
}

#[tokio::test]
async fn changing_password_works() {
    let app = spawn_app().await;

    let new_password = Uuid::new_v4().to_string();

    // LOGIN
    let response = app
        .post_login(&serde_json::json!({
            "username": app.test_user.username,
            "password": app.test_user.password,
        }))
        .await;
    assert_is_redirect_to(&response, "/admin/dashboard");

    let response = app
        .post_change_password(&serde_json::json!({
            "current_password": app.test_user.password,
            "new_password": &new_password,
            "new_password_check": &new_password,
        }))
        .await;
    assert_is_redirect_to(&response, "/admin/dashboard");

    let html_page = app.get_change_password_html().await;
    assert!(html_page.contains("<p><i>Your password has been changed.</i></p>"));

    let response = app.post_logout().await;
    assert_is_redirect_to(&response, "/login");

    let html_page = app.get_login_html().await;
    assert!(html_page.contains("<p><i>You have successfully logged out.</i></p>"));

    let response = app
        .post_login(&serde_json::json!({
            "username": app.test_user.username,
            "password": &new_password,
        }))
        .await;
    assert_is_redirect_to(&response, "/admin/dashboard");
}

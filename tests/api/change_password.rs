use crate::helpers::{assert_is_redirect_to, spawn_app};
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
    let response = app .post_change_password(&serde_json::json!({
"current_password": Uuid::new_v4().to_string(), "new_password": &new_password, "new_password_check": &new_password,
})) .await;
    // Assert
    assert_is_redirect_to(&response, "/login");
}

#[tokio::test]
async fn new_password_fields_must_match() {
    // Arrange
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();
    let another_new_password = Uuid::new_v4().to_string();
    // Act - Part 1 - Login
    app.post_login(&serde_json::json!({ "username": &app.test_user.username, "password": &app.test_user.password
})) .await;
    // Act - Part 2 - Try to change password
    let response = app
        .post_change_password(&serde_json::json!({
                    "current_password": &app.test_user.password,
                    "new_password": &new_password,
                    "new_password_check": &another_new_password,
        }))
        .await;
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
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();
    let wrong_password = Uuid::new_v4().to_string();
    // Act - Part 1 - Login
    app.post_login(&serde_json::json!({ "username": &app.test_user.username, "password": &app.test_user.password
    })) .await;
    // Act - Part 2 - Try to change password
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
async fn new_password_must_be_valid_length() {
    let app = spawn_app().await;
    let new_password_short = "short"; // Less than 12 characters
    let new_password_long = &"a".repeat(129); // More than 128 characters
    let current_password = &app.test_user.password;

    // Act - Part 1 - Login
    app.post_login(
        &serde_json::json!({ "username": &app.test_user.username, "password": current_password
        }),
    )
    .await;

    // Act - Part 2 - Try to change password to a short one
    let response_short = app
        .post_change_password(&serde_json::json!({
                    "current_password": current_password,
                    "new_password": &new_password_short,
                    "new_password_check": &new_password_short,
        }))
        .await;

    // Assert
    assert_is_redirect_to(&response_short, "/admin/password");
    // Act - Part 3 - Follow the redirect
    let html_page_short = app.get_change_password_html().await;
    assert!(html_page_short.contains(
        "<p><i>Password must be longer than 12 characters but shorter than 128 characters.</i></p>"
    ));

    // Act - Part 4 - Try to change password to a long one
    let response_long = app
        .post_change_password(&serde_json::json!({
                    "current_password": current_password,
                    "new_password": &new_password_long,
                    "new_password_check": &new_password_long,
        }))
        .await;

    // Assert
    assert_is_redirect_to(&response_long, "/admin/password");
    // Act - Part 5 - Follow the redirect
    let html_page_long = app.get_change_password_html().await;
    assert!(html_page_long.contains(
        "<p><i>Password must be longer than 12 characters but shorter than 128 characters.</i></p>"
    ));
}

#[tokio::test]
async fn changing_password_works() {
    // Arrange
    let app = spawn_app().await;
    let new_password = Uuid::new_v4().to_string();
    // Act - Part 1 - Login
    let login_body = serde_json::json!({ "username": &app.test_user.username, "password": &app.test_user.password
    });
    let response = app.post_login(&login_body).await;
    assert_is_redirect_to(&response, "/admin/dashboard");
    // Act - Part 2 - Change password
    let response = app
        .post_change_password(&serde_json::json!({
                    "current_password": &app.test_user.password,
                    "new_password": &new_password,
                    "new_password_check": &new_password,
        }))
        .await;
    assert_is_redirect_to(&response, "/admin/password");
    // Act - Part 3 - Follow the redirect
    let html_page = app.get_change_password_html().await;
    assert!(html_page.contains("<p><i>Your password has been changed.</i></p>"));
    // Act - Part 4 - Logout
    let response = app.post_logout().await;
    assert_is_redirect_to(&response, "/login");
    // Act - Part 5 - Follow the redirect
    let html_page = app.get_login_html().await;
    assert!(html_page.contains("<p><i>You have successfully logged out.</i></p>"));
    // Act - Part 6 - Login using the new password
    let login_body = serde_json::json!({ "username": &app.test_user.username, "password": &new_password
    });
    let response = app.post_login(&login_body).await;
    assert_is_redirect_to(&response, "/admin/dashboard");
}

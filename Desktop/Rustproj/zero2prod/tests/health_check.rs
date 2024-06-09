use std::net::TcpListener;



#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    // we need "reqwest" to do requests against application
    let client = reqwest::Client::new();
    // send a get request and await response
    let response = client
    .get(&format!("{}/health_check", &address))
    .send()
    .await
    .expect("Failed to execute request.");

    assert!(response.status().is_success()); // ssert that status code is 200 (Succesful GET request)
    assert_eq!(Some(0), response.content_length()); // assert that content length is 0
 }

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    // Retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind random port");
    let _ = tokio::spawn(server);
    // return the application address
    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_form_data() {
    // Arrange Server address and Client 
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
    .post(&format!("{}/subscriptions", &app_address))
    .header("Content-Type", "application/x-www-form-urlencoded")
    .body(body)
    .send()
    .await
    .expect("Failed to execute request.");

    // Assert
    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_400_when_data_is_missing() {
    // Arrange Server address and Client 
    let app_address = spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email")
    ];
    // Act
    for (invalid_body, error_message) in test_cases {
        // Act
        let response = client
        .post(&format!("{}/subscriptions", &app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(invalid_body)
        .send()
        .await
        .expect("Failed to execute request.");
        // Assert
        assert_eq!(
            400,
            response.status().as_u16(),
            // Custom error message
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );

    }
}
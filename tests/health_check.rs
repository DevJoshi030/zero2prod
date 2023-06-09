use std::net::TcpListener;

#[tokio::test]
async fn health_check() {
    let address = spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{address}/health_check"))
        .send()
        .await
        .expect("Failed to execute a reqwest");

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(14))
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to find a port");

    let port = listener.local_addr().unwrap().port();
    let server = zero2prod::run(listener).expect("Failed to bind address");
    tokio::spawn(server);

    format!("http://127.0.0.1:{port}")
}

#[derive(From)]
struct Test<'a> {
    email: &'a str,
    name: &'a str,
}

#[tokio::test]
async fn subscribe_returns_200_for_valid_data() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    println!("{address}");

    let t = Test {
        email: "ursula_le_guin@gmail.com",
        name: "le guin",
    };

    let body = t;
    let response = client
        .post(format!("{address}/subscriptions"))
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await
        .expect("Failed to execute reqwest");

    assert_eq!(response.status().as_u16(), 200);
}

#[tokio::test]
async fn subscribe_returns_400_when_data_is_missing() {
    let address = spawn_app();
    let client = reqwest::Client::new();

    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(format!("{address}/subscriptions"))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute reqwest");

        assert_eq!(
            400,
            response.status().as_u16(),
            // Additional customised error message on test failure
            "The API did fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

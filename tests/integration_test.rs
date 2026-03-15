/// Integration tests for RustView server HTTP endpoints.
use rustview::server;
use rustview::ui::Ui;

fn test_app(ui: &mut Ui) {
    let name = ui.text_input("Name", "World");
    ui.write(format!("Hello, {}!", name));
    let n = ui.int_slider("Count", 0..=100);
    ui.progress(n as f64 / 100.0);
    if ui.checkbox("Show error", false) {
        ui.error("Test error");
    }
    if ui.button("Click me") {
        ui.write("Button clicked!");
    }
    ui.markdown("# Test Markdown");
}

#[tokio::test]
async fn test_index_returns_html() {
    let router = server::build_router(test_app);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::serve(listener, router).await.unwrap();
    });

    // Give server time to start
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let resp = reqwest::get(format!("http://{}/", addr)).await.unwrap();
    assert_eq!(resp.status(), 200);

    let body = resp.text().await.unwrap();
    assert!(body.contains("<!DOCTYPE html>"));
    assert!(body.contains("RustView App"));
    assert!(body.contains("rustview-root"));
    assert!(body.contains("Hello, World!"));
    assert!(body.contains("SESSION_ID"));
    // Should have widgets
    assert!(body.contains("data-widget-type"));
    assert!(body.contains("text_input"));
    assert!(body.contains("slider"));
    assert!(body.contains("checkbox"));
    assert!(body.contains("button"));
    // Should have CSS
    assert!(body.contains("rustview-widget"));
    // Should have JS shim
    assert!(body.contains("EventSource"));
}

#[tokio::test]
async fn test_event_endpoint_accepts_json() {
    let router = server::build_router(test_app);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::serve(listener, router).await.unwrap();
    });

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    // First get the page to create a session
    let body = reqwest::get(format!("http://{}/", addr))
        .await
        .unwrap()
        .text()
        .await
        .unwrap();

    // Extract session ID from HTML
    let sid_start = body.find("SESSION_ID = \"").unwrap() + 14;
    let sid_end = body[sid_start..].find('"').unwrap() + sid_start;
    let session_id = &body[sid_start..sid_end];

    // Send a widget event
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("http://{}/event", addr))
        .json(&serde_json::json!({
            "sid": session_id,
            "widget_id": "w-Name-1",
            "value": "Alice"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
}

#[tokio::test]
async fn test_event_invalid_session_returns_not_found() {
    let router = server::build_router(test_app);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        axum::serve(listener, router).await.unwrap();
    });

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let client = reqwest::Client::new();
    let resp = client
        .post(format!("http://{}/event", addr))
        .json(&serde_json::json!({
            "sid": "00000000-0000-0000-0000-000000000000",
            "widget_id": "test-widget",
            "value": "test"
        }))
        .send()
        .await
        .unwrap();

    assert_eq!(resp.status(), 404);
}

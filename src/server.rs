use axum::{routing::get, Router};
use product_api_rust::products;

#[tokio::main]
pub async fn start() {
    let app = Router::new().route(
        "/:product_id",
        get(products::handlers::find_one_product_handler),
    );

    println!("server is running on {:?}", 3000);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

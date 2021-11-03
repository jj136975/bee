pub mod endpoints;
pub mod types;

use bee_storage_sled::{
    storage::Storage,
    config::SledConfigBuilder,
};

use axum::{
    AddExtensionLayer,
    handler::get,
    response::Html,
    Router
};
use std::{
    net::SocketAddr,
    sync::{Arc, Mutex},
};

pub struct AppStorage {
    storage: Mutex<Storage>,
}

#[tokio::main]
async fn main() {
    let sled_config = SledConfigBuilder::new().finish();
    let storage;

    match Storage::new(sled_config) {
        Err(e) => {
            println!("Error creating storage config {:?}", e);
            return;
        }
        Ok(conf) => storage = conf,
    }
    let app_storage = Arc::new(AppStorage {storage: Mutex::new(storage)});
    let app = Router::new()
        .route("/", get(handler))
        .nest("/api", endpoints::routes::api::api_routes())
        .layer(AddExtensionLayer::new(app_storage));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

pub mod messages;

use axum::{
    handler::get,
    Router,
    response::Html,
    routing::BoxRoute
};

pub fn api_routes() -> Router<BoxRoute> {
    Router::new()
        .route("/", get(handler))
        //.nest("/test", test::api_routes())
        .nest("messages", messages::api_routes())
        .boxed()
}

async fn handler() -> Html<&'static str> {
    Html("<h1>You are in /v1</h1>")
}
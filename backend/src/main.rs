use actix_cors::Cors;
use actix_web::{
    App, HttpResponse, HttpServer, Responder, get, http::header, middleware::Logger, web
};

use backend::listener;

mod state;
pub use state::*;

use dotenv::dotenv;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    // Initializes the env loggin system for this app
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    log::info!("Starting HTTP server at http://localhost:8080");

    // shared state
    let app_state = web::Data::new(AppState {
        web_socket_subscription_id: None.into(),
    });

    // Spawn webSocket Listener in Background
    tokio::spawn(listener());

    HttpServer::new(move || {
        // App instance is used for registering routes for resources and middleware, also stores application state shared across all handlers.
        App::new()
            // Register the state when initializing the app
            .app_data(app_state.clone())
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:5173/")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(Logger::default())
            .service(hello)
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}

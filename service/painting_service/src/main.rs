

use serde::{Deserialize, Serialize};
use actix_web::{web, HttpResponse, HttpServer, App};
use uuid::Uuid;
use dotenv::dotenv;
use std::env;


#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}




async fn hello_world() -> HttpResponse
{
    HttpResponse::Ok().json("Hello World")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());

    println!("Server running at http://{}:{}", host, port);
    // Create and run the HTTP server
    HttpServer::new(|| {
        App::new()
            // Define a route: GET requests to "/" will be handled by hello_world
            .route("/", web::get().to(hello_world))
    })
    // Bind the server to listen on localhost:8080
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
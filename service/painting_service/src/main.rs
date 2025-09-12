

use serde::{Deserialize, Serialize};
use actix_web::{web, HttpResponse, HttpServer, App};
use uuid::Uuid;
use dotenv::dotenv;
use std::env;


#[derive(Serialize, Deserialize, Debug)]
struct Message {
    message: String,
}




#[derive(Serialize, Deserialize, Debug)]
struct NumberMessage {
    number: i32,
}


async fn hello_world() -> HttpResponse
{
    let new_message: Message = Message{
        message: String::from("Hello World"),
    };

    HttpResponse::Ok().json(new_message)
}

async fn callAndResponse(message: web::Path<String>, query: web::Query<NumberMessage>) -> HttpResponse
{
    let new_message: Message = Message{
        message: format!("{} and input number is {}", message, query.number),
    };

    HttpResponse::Ok().json(new_message)
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
            .route("/", web::get().to(hello_world)) //http://localhost:8080/
            .route("/{message}", web::get().to(callAndResponse)) //http://localhost:8080/hello?number=354886
    })
    // Bind the server to listen on localhost:8080
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}


use serde::{Deserialize, Serialize};
use actix_web::{rt, web, HttpResponse, HttpRequest, HttpServer, App, Error};
use actix_ws::{AggregatedMessage};
use uuid::Uuid;
use dotenv::dotenv;
use std::env;

mod canvas;
mod messages;

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

async fn echo(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let (res, mut session, stream) = actix_ws::handle(&req, stream)?;

    let mut stream = stream
        .aggregate_continuations()
        // aggregate continuation frames up to 1MiB
        .max_continuation_size(2_usize.pow(20));

    // start task but don't wait for it
    rt::spawn(async move {
        // receive messages from websocket
        while let Some(msg) = stream.recv().await {
            match msg {
                Ok(AggregatedMessage::Text(text)) => {
                    // echo text message
                    session.text(text).await.unwrap();
                }

                Ok(AggregatedMessage::Binary(bin)) => {
                    // echo binary message
                    session.binary(bin).await.unwrap();
                }

                Ok(AggregatedMessage::Ping(msg)) => {
                    // respond to PING frame with PONG frame
                    session.pong(&msg).await.unwrap();
                }

                _ => {}
            }
        }
    });

    // respond immediately with response connected to WS session
    Ok(res)
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
            .route("/echo", web::get().to(echo)) //http://localhost:8080/echo
            .route("/{message}", web::get().to(callAndResponse)) //http://localhost:8080/hello?number=354886
    })
    // Bind the server to listen on localhost:8080
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
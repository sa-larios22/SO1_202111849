use actix_web::{post, App, HttpResponse, HttpServer, Responder, web};
use serde::Deserialize;
use std::env;

#[derive(Deserialize)]
struct Tweet {
    Description: String,
    Country: String,
    Weather: String,
}

#[get("/")]
async fn get_status() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[post("/input")]
async fn receive_tweet(tweet: web::Json<Tweet>) -> impl Responder {
    // Imprime el tweet recibido
    println!(
        "Descripción: {}, País: {}, Clima: {}",
        tweet.Description, tweet.Country, tweet.Weather
    );

    // Reenvío opcional al servicio en Go (puede ser gRPC o REST)
    // if let Ok(go_api_url) = env::var("GO_API_URL") {
    //     let client = reqwest::Client::new();
    //     let _ = client.post(&go_api_url)
    //         .json(&*tweet)
    //         .send()
    //         .await;
    // }

    HttpResponse::Ok().body("Tweet recibido")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    println!("🚀 Servidor Rust escuchando en http://0.0.0.0:{}", port);

    // let cors = CorsOptions::default()
    //     .allowed_origins(AllowedOrigins::all())
    //     .to_cors()
    //     .expect("Failed to create CORS options");

    HttpServer::new(|| {
        App::new()
            .service(receive_tweet)
            .service(get_status)
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
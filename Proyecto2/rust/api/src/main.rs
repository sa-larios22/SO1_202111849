use actix_web::{post, get, App, HttpResponse, HttpServer, Responder, web};
use serde::{Serialize, Deserialize,};
use reqwest::Client;
use std::env;

#[derive(Serialize, Deserialize)]
struct Tweet {
    description: String,
    country: String,
    weather: String,
}

#[get("/")]
async fn get_status() -> impl Responder {
    HttpResponse::Ok().body("Estado del servidor de Rust: Activo")
}

#[post("/input")]
async fn receive_tweet(tweet: web::Json<Tweet>) -> impl Responder {
    // Imprime el tweet recibido
    println!(
        "Descripción: {}, País: {}, Clima: {}",
        tweet.description, tweet.country, tweet.weather
    );

    // --------------------------------------------------
    let client = Client::new();

    // Reenvío al servicio en Go con una variable de entorno
    let go_service_url = match env::var("GO_SERVICE_URL") {
        Ok(url) => url,
        Err(_) => {
            println!("⚠️  GO_SERVICE_URL no definida, usando http://localhost:8081/input por defecto.");
            "http://localhost:8081/input".to_string()
        }
    };

    println!("Reenviando a {}", go_service_url);

    let result = client.post(&go_service_url)
        .json(&*tweet)
        .send()
        .await;

    match result {
        Ok(response) => {
            if response.status().is_success() {
                println!("Tweet reenviado exitosamente al servicio en Go");
            } else {
                println!("Fallo en el reenvío. Código HTTP: {}", response.status());
            }
        }
        Err(err) => {
            println!("Error al conectar con el servicio en Go:");
            println!("{:#}", err);
        }
    }

    HttpResponse::Ok().json(&*tweet)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = 8080;
    println!("Servidor Rust escuchando en http://0.0.0.0:{}", port);

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
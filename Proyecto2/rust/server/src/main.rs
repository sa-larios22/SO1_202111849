use rocket::{routes, serde::json::Json};
use rocket::post;
use rocket::get;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use rocket::config::SecretKey;
use rocket_cors::{AllowedOrigins, CorsOptions};

#[derive(Debug, Serialize, Deserialize)]
struct Tweet {
    Description: String,
    Country: String,
    Weather: String,
}

#[post("/input", data = "<data>")]
async fn tweet(data: Json<Data>) -> String {
    let client = Client::new();

    // print
    println!("Client received data. Sending to server: description: {}, country: {}, weather: {}", data.Description, data.Country, data.Weather);

    let server_url = "http://localhost:3001/input";
    let response = client.post(server_url).
        json(&data.into_inner()).
        send().
        await;

    match response {
        Ok(_) => "Data sent successfully".to_string(),
        Err(e) => format!("Error sending data: {}", e),
    }
}

#[rocket::main]
async fn main() {
    let secret_key = SecretKey::generate();

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .to_cors()
        .expect("Failed to create CORS options");

    let config = rocket::Config {
        address: "0.0.0.0".parse().unwrap(),
        port: 3000,
        secret_key: secret_key.unwrap(), // Desempaqueta la clave secreta generada
        ..rocket::Config::default()
    };

    rocket::custom(config)
    .attach(cors)
    .mount("/", rocket::routes![vote, get_data])
    .launch()
    .await
    .unwrap();
}
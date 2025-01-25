use actix_cors::Cors;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

mod api_request;
use api_request::create_gemini_api_post;

mod rr_json;
use rr_json::Response;

async fn use_gemini_api_post_function(req_body: String) -> impl Responder {
    match create_gemini_api_post(req_body).await {
        Ok(response) => {
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "Failed to response body".to_string());

            let mut real_message = String::new();

            match serde_json::from_str::<Response>(&body) {
                Ok(response) => {
                    real_message = response
                        .candidates
                        .get(0)
                        .and_then(|candidate| candidate.content.parts.get(0))
                        .map_or("No content found".to_string(), |part| part.text.clone());
                }
                Err(err) => {
                    println!("Error: {}", err);
                }
            }

            HttpResponse::Ok().body(real_message)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {e}")),
    }
}

#[actix_web::get("/")]
async fn hello_there() -> impl Responder {
    HttpResponse::Ok().body("WELCOME TO ADMIN PANEL.")
}

async fn gemini_route_status() -> impl Responder {
    HttpResponse::Ok()
        .body("The route is available. Start with going to /gemini address and make a post request.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("The server is started at: 127.0.0.1:5000");
    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec!["Authorization", "Accept"])
                    .allowed_header("content-type")
                    .max_age(3600)
            )
            .service(
                web::scope("/gemini")
                    .route("/", web::get().to(gemini_route_status))
                    .route("/", web::post().to(use_gemini_api_post_function)),
            )
            .service(hello_there)
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}
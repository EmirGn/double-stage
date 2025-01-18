use actix_web::{web, App, HttpResponse, HttpServer, Responder};
mod create_api;
use create_api::create_gemini_api_post;

async fn use_gemini_api_post_function(req_body: String) -> impl Responder {
    match create_gemini_api_post(req_body).await {
        Ok(response) => {
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "Failed to response body".to_string());
            HttpResponse::Ok().body(body)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {e}")),
    }
}

async fn gemini_route_status() -> impl Responder {
    HttpResponse::Ok()
        .body("The route is available start with going to /gemini address and make a post request.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            web::scope("/gemini")
                .route("/", web::get().to(gemini_route_status))
                .route("/", web::post().to(use_gemini_api_post_function)),
        )
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}

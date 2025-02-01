use actix_cors::Cors;
use actix_web::{body::None, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use serde_json;

mod api_request;
use api_request::create_gemini_api_post;

mod rr_json;
use rr_json::Response;

mod database;
use database::{create_chat, establish_connection, show_chats};

#[derive(Deserialize)]
struct CreateChatPayload {
    input_value: String,
    chat_id: String
}

#[allow(unused_variables)]
async fn use_gemini_api_post_function(req_body: &String) -> Result<String, None>{
    let mut real_message = String::new();
    match create_gemini_api_post(req_body).await {
        Ok(response) => {
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "Failed to response body".to_string());

            match serde_json::from_str::<Response>(&body) {
                Ok(response) => {
                    real_message = response
                        .candidates
                        .get(0)
                        .and_then(|candidate| candidate.content.parts.get(0))
                        .map_or("No content found".to_string(), |part| part.text.clone());
                }
                Err(err) => {
                    let error = err;
                }
            }

        }
        Err(err) => {
            let error = err;
        }
    }

    Ok(real_message)
}

#[actix_web::get("/")]
async fn hello_there() -> impl Responder {
    HttpResponse::Ok().body("WELCOME TO ADMIN PANEL.")
}

#[actix_web::get("/c")]
async fn get_all_chat_history() -> impl Responder {
    let connection = &mut establish_connection();
    let chat_titles = show_chats(connection, None).unwrap();
    HttpResponse::Ok()
        .json(chat_titles)
}

async fn _get_unit_chat_history() -> impl Responder {
    HttpResponse::Ok()
        .body("Hi")
}

#[actix_web::patch("/c/{id}")]
async fn update_unit_chat_history(_chat_id: web::Path<String>) -> impl Responder {
    // Database operations for updating the history.
    HttpResponse::Ok()
        .body("Chat history is updated.")
}

#[actix_web::post("/c")]
async fn create_new_chat(payload: web::Json<CreateChatPayload>) -> impl Responder {
    let payload_tuple = (&payload.input_value, &payload.chat_id);
    let connection = &mut establish_connection();
    
    let model_output = use_gemini_api_post_function(payload_tuple.0).await.unwrap();

    create_chat(connection, payload_tuple.1, None, payload_tuple.0, Some(&model_output));

    HttpResponse::Ok()
        .body(format!("Chat is added to the db. The details are: {:?}", payload_tuple))
}

#[actix_web::delete("/c/{id}")]
async fn delete_chat(chat_id: web::Path<String>) -> impl Responder {
    // Database operations for deleting the chat
    HttpResponse::Ok()
        .body(format!("Chat {} is deleted.", chat_id))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("The server is started at: 127.0.0.1:5000");
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec!["Authorization", "Accept"])
                    .allowed_header("content-type")
                    .max_age(3600)
            )
            .service(hello_there)
            .service(create_new_chat)
            .service(get_all_chat_history)
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}
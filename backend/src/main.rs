use actix_cors::Cors;
use actix_web::{body::None, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use serde_json;

mod api_request;
use api_request::create_gemini_api_post;

mod rr_json;
use rr_json::Response;

mod database;
use database::{
    create_new_chat_handler_database, establish_connection, get_all_chat_history_handler_database,
    get_unit_chat_history_handler_database, update_unit_chat_history_handler_database, delete_chat_handler_database
};

#[derive(Deserialize)]
struct CreateChatPayload {
    input_value: String,
    chat_id: String
}

#[derive(Deserialize)]
struct UpdateChatPayload {
    new_history: String,
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
    let chat_titles = get_all_chat_history_handler_database(connection, None).unwrap();
    HttpResponse::Ok()
        .json(chat_titles)
}

#[actix_web::post("/c")]
async fn create_new_chat(payload: web::Json<CreateChatPayload>) -> impl Responder {
    let payload_tuple = (&payload.input_value, &payload.chat_id);
    let connection = &mut establish_connection();
    
    let model_output = use_gemini_api_post_function(payload_tuple.0).await.unwrap();

    let _ = create_new_chat_handler_database(connection, payload_tuple.1, None, payload_tuple.0, Some(&model_output));

    HttpResponse::Ok()
        .body(format!("Chat is added to the db. The details are: {:?}", payload_tuple))
}

#[actix_web::get("/c/{id}")]
async fn get_unit_chat_history(chat_id: web::Path<String>) -> impl Responder {
    let id = chat_id.into_inner();
    let connection = &mut establish_connection();
    match get_unit_chat_history_handler_database(connection, &id) {
        Ok(chat) => HttpResponse::Ok().json(chat),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[actix_web::patch("/c/{id}")]
async fn update_unit_chat_history(chat_id: web::Path<String>, payload: web::Json<UpdateChatPayload>) -> impl Responder {
    let id = chat_id.into_inner();
    let connection = &mut establish_connection();
    match update_unit_chat_history_handler_database(connection, &id, &payload.new_history) {
        Ok(_) => HttpResponse::Ok().body("Chat history is updated."),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
}

#[actix_web::delete("/c/{id}")]
async fn delete_chat(chat_id: web::Path<String>) -> impl Responder {
    let id = chat_id.into_inner();
    let connection = &mut establish_connection();
    match delete_chat_handler_database(connection, &id) {
        Ok(_) => HttpResponse::Ok().body(format!("Chat {} is deleted.", id)),
        Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
    }
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
            .service(get_unit_chat_history)
            .service(update_unit_chat_history)
            .service(delete_chat)
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}
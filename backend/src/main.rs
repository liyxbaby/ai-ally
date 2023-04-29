use actix_web::{get, post, delete, put, App, web, HttpResponse, HttpServer};
use futures_util::StreamExt as _;
mod database;
use database::{Database, Message, NewMessage, CompanionView, UserView, ConfigModify};
mod long_term_mem;
use long_term_mem::LongTermMem;
mod dialogue_tuning;
use dialogue_tuning::DialogueTuning;
mod character_card;
use character_card::CharacterCard;
use serde::Deserialize;
mod llm;
use crate::llm::prompt;

use std::fs;
use std::fs::File;
use std::io::{Write, Read};

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body(include_str!("../../dist/index.html"))
}

#[get("/assets/index-4rust.js")]
async fn js() -> HttpResponse {
    HttpResponse::Ok().content_type("application/javascript").body(include_str!("../../dist/assets/index-4rust.js"))
}

#[get("/assets/index-4rust2.js")]
async fn js2() -> HttpResponse {
    HttpResponse::Ok().content_type("application/javascript").body(include_str!("../../dist/assets/index-4rust2.js"))
}

#[get("/assets/index-4rust.css")]
async fn css() -> HttpResponse {
    HttpResponse::Ok().content_type("text/css").body(include_str!("../../dist/assets/index-4rust.css"))
}

#[get("/ai_companion_logo.jpg")]
async fn project_logo() -> HttpResponse {
    HttpResponse::Ok().content_type("image/jpeg").body(&include_bytes!("../../dist/ai_companion_logo.jpg")[..])
}

#[get("/assets/companion_avatar-4rust.jpg")]
async fn companion_avatar_img() -> HttpResponse {
    HttpResponse::Ok().content_type("image/jpeg").body(&include_bytes!("../../dist/assets/companion_avatar-4rust.jpg")[..])
}

#[get("/assets/avatar.png")]
async fn companion_avatar_custom() -> actix_web::Result<actix_web::HttpResponse> {
    match File::open("assets/avatar.png") {
        Ok(mut file) => {
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;

            Ok(actix_web::HttpResponse::Ok()
                .content_type("image/png")
                .body(buffer))
        }
        Err(_) => Err(actix_web::error::ErrorNotFound("File not found")),
    }
}


//              API


//              Message

#[derive(serde::Deserialize)]
struct MessageQuery {
    start_index: Option<usize>,
    limit: Option<usize>,
}

#[get("/api/message")]
async fn message(query_params: web::Query<MessageQuery>) -> HttpResponse {
    let start_index: usize = query_params.start_index.unwrap_or(0);

    // 50 Messages is the max
    let limit: usize = query_params.limit.unwrap_or(15).min(50);

    // query to database, and return messages
    let messages: Vec<Message> = match Database::get_x_messages(limit, start_index) {
        Ok(v) => v,
        Err(e) => {
            println!("Failed to get messages from database: {}", e);
            return HttpResponse::InternalServerError().body("Error while getting messages from database, check logs for more information");
        },
    };
    let messages_json = serde_json::to_string(&messages).unwrap_or(String::from("Error serializing messages as JSON"));
    HttpResponse::Ok().body(messages_json)
}

#[post("/api/message")]
async fn message_post(received: web::Json<NewMessage>) -> HttpResponse {
    match Database::insert_message(received.into_inner()) {
        Ok(_) => HttpResponse::Ok().body("Message added!"),
        Err(e) => {
            println!("Failed to add message: {}", e);
            HttpResponse::InternalServerError().body("Error while adding message, check logs for more information")
        }
    }
}

#[delete("/api/message")]
async fn clear_messages() -> HttpResponse {
    match Database::erase_messages() {
        Ok(_) => HttpResponse::Ok().body("Chat log cleared!"),
        Err(e) => {
            println!("Failed to clear chat log: {}", e);
            HttpResponse::InternalServerError().body("Error while clearing chat log, check logs for more information")
        }
    }
}

#[get("/api/message/{id}")]
async fn message_id(id: web::Path<i32>) -> HttpResponse {
    let msg: Message = match Database::get_message(*id) {
        Ok(v) => v,
        Err(e) => {
            println!("Failed to get message at id {}: {}", id, e);
            return HttpResponse::InternalServerError().body(format!("Error while getting message at id {}, check logs for more information", id));
        }
    };
    let message_json = serde_json::to_string(&msg).unwrap_or(String::from("Error serializing message as JSON"));
    HttpResponse::Ok().body(message_json)
}

#[put("/api/message/{id}")]
async fn message_put(id: web::Path<i32>, received: web::Json<NewMessage>) -> HttpResponse {
    match Database::edit_message(*id, received.into_inner()) {
        Ok(_) => HttpResponse::Ok().body(format!("Message edited at id {}!", id)),
        Err(e) => {
            println!("Failed to edit me
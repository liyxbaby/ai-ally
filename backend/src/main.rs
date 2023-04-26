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
    HttpResponse::Ok().body(include_str!("../../dist/index.
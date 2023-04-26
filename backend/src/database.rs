
use rusqlite::{Connection, Error, Result, ToSql};
use rusqlite::types::{FromSql, FromSqlError, ValueRef, ToSqlOutput};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Local};

use crate::character_card::CharacterCard;


#[derive(Serialize, Deserialize)]
pub struct Message {
    pub id: i32,
    pub ai: bool,
    pub content: String,
    pub created_at: String,
}

pub fn get_current_date() -> String {
    let local: DateTime<Local> = Local::now();
    local.format("%A %d.%m.%Y %H:%M").to_string()
}

pub fn contains_time_question(text: &str) -> bool {
    let time_related_keywords = ["time", "date", "hour", "day", "month", "year", "minute", "second", "morning", "afternoon", "evening", "night"];
    for keyword in &time_related_keywords {
        if text.contains(keyword) {
            return true;
        }
    }
    false
}

#[derive(Serialize, Deserialize)]
pub struct NewMessage {
    pub ai: bool,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
pub struct Companion {
    pub id: i32,
    pub name: String,
    pub persona: String,
    pub example_dialogue: String,
    pub first_message: String,
    pub long_term_mem: usize,
    pub short_term_mem: usize,
    pub roleplay: bool,
    pub dialogue_tuning: bool,
    pub avatar_path: String,
}

#[derive(Serialize, Deserialize)]
pub struct CompanionView {
    pub name: String,
    pub persona: String,
    pub example_dialogue: String,
    pub first_message: String,
    pub long_term_mem: usize,
    pub short_term_mem: usize,
    pub roleplay: bool,
    pub dialogue_tuning: bool,
    pub avatar_path: String,
}

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub persona: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserView {
    pub name: String,
    pub persona: String,
}

#[derive(PartialEq, Serialize, Deserialize)]
pub enum Device {
    CPU,
    GPU,
    Metal,
}

impl FromSql for Device {
    fn column_result(value: ValueRef<'_>) -> Result<Self, FromSqlError> {
        match value {
            ValueRef::Text(i) => {
                match std::str::from_utf8(i) {
                    Ok(s) => {
                        match s {
                            "CPU" => Ok(Device::CPU),
                            "GPU" => Ok(Device::GPU),
                            "Metal" => Ok(Device::Metal),
                            _ => Err(FromSqlError::OutOfRange(0)),
                        }
                    }
                    Err(e) => Err(FromSqlError::Other(Box::new(e))),
                }
            }
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl ToSql for Device {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        match self {
            Device::CPU => Ok(ToSqlOutput::from("CPU")),
            Device::GPU => Ok(ToSqlOutput::from("GPU")),
            Device::Metal => Ok(ToSqlOutput::from("Metal")),
        }
    }
}

#[derive(PartialEq, Serialize, Deserialize)]
pub enum PromptTemplate {
    Default,
    Llama2,
    Mistral
}

impl FromSql for PromptTemplate {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> Result<Self, FromSqlError> {
        match value {
            ValueRef::Text(i) => {
                match std::str::from_utf8(i) {
                    Ok(s) => {
                        match s {
                            "Default" => Ok(PromptTemplate::Default),
                            "Llama2" => Ok(PromptTemplate::Llama2),
                            "Mistral" => Ok(PromptTemplate::Mistral),
                            _ => Err(FromSqlError::OutOfRange(0)),
                        }
                    }
                    Err(e) => Err(FromSqlError::Other(Box::new(e))),
                }
            }
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl ToSql for PromptTemplate {
    fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
        match self {
            PromptTemplate::Default => Ok(ToSqlOutput::from("Default")),
            PromptTemplate::Llama2 => Ok(ToSqlOutput::from("Llama2")),
            PromptTemplate::Mistral => Ok(ToSqlOutput::from("Mistral")),
        }
    }
}


/*
struct Config {
    id: i32,
    device: Device,
    llm_model_path: String,
    gpu_layers: usize,
    prompt_template: PromptTemplate
}
*/

#[derive(Serialize, Deserialize)]
pub struct ConfigView {
    pub device: Device,
    pub llm_model_path: String,
    pub gpu_layers: usize,
    pub prompt_template: PromptTemplate
}

#[derive(Serialize, Deserialize)]
pub struct ConfigModify {
    pub device: String,
    pub llm_model_path: String,
    pub gpu_layers: usize,
    pub prompt_template: String
}

pub struct Database {}

impl Database {
    pub fn new() -> Result<usize> {
        let con = Connection::open("companion_database.db")?;
        con.execute(
            "CREATE TABLE IF NOT EXISTS messages (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                ai BOOLEAN,
                content TEXT,
                created_at TEXT
            )", []
        )?;
        con.execute(
            "CREATE TABLE IF NOT EXISTS companion (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT,
                persona TEXT,
                example_dialogue TEXT,
                first_message TEXT,
                long_term_mem INTEGER,
                short_term_mem INTEGER,
                roleplay BOOLEAN,
                dialogue_tuning BOOLEAN,
                avatar_path TEXT
            )", []
        )?;
        con.execute(
            "CREATE TABLE IF NOT EXISTS user (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT,
                persona TEXT,
                avatar_path TEXT
            )", []
        )?;
        con.execute(
            "CREATE TABLE IF NOT EXISTS config (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                device TEXT,
                llm_model_path TEXT,
                gpu_layers INTEGER,
                prompt_template TEXT
            )", []
        )?;
        if Database::is_table_empty("companion", &con)? {
            con.execute(
                "INSERT INTO companion (name, persona, example_dialogue, first_message, long_term_mem, short_term_mem, roleplay, dialogue_tuning, avatar_path) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
                &[
                    "Assistant",
                    "{{char}} is an artificial intelligence chatbot designed to help {{user}}. {{char}} is an artificial intelligence created in ai-companion backend",
                    "{{user}}: What is ai-companion?\n{{char}}: AI Companion is a open-source project, wrote in Rust, Typescript and React, that aims to provide users with their own personal AI chatbot on their computer. It allows users to engage in friendly and natural conversations with their AI, creating a unique and personalized experience. This software can also be used as a backend or API for other projects that require a personalised AI chatbot. Very light size, simple installation, simple configuration, quick cold start and ease of use are some of the strengths of AI Companion in comparison to other similar projects.\n{{user}}: Can you tell me about the creator of ai-companion?\n{{char}}: the creator of the ai-companion program is 'Hubert Kasperek', he is a young programmer from Poland who is mostly interested in web development and computer science concepts, he has account on GitHub under nickname \"Hukasx0\"",
                    "Hello {{user}}, how can i help you today?",
                    "2",
                    "5",
                    "1",
                    "1",
                    "/assets/companion_avatar-4rust.jpg"
                ]
            )?;
        }
        if Database::is_table_empty("user", &con)? {
            con.execute(
                "INSERT INTO user (name, persona, avatar_path) VALUES (?, ?, ?)",
                &[
                    "User",
                    "{{user}} is chatting with {{char}} using ai-companion web user interface",
                    "/assets/user_avatar-4rust.jpg"
                ]
            )?;
        }
        if Database::is_table_empty("messages", &con)? {
            struct CompanionReturn {
                name: String,
                first_message: String
            }
            let companion_data = con.query_row("SELECT name, first_message FROM companion", [], |row| {
                Ok(CompanionReturn {
                    name: row.get(0)?,
                    first_message: row.get(1)?
                   }
                )
            })?;
            let user_name: String = con.query_row("SELECT name, persona FROM user LIMIT 1", [], |row| {
                Ok(row.get(0)?)
            })?;
            con.execute(
                "INSERT INTO messages (ai, content, created_at) VALUES (?, ?, ?)",
                &[
                    "1",
                    &companion_data.first_message.replace("{{char}}", &companion_data.name).replace("{{user}}", &user_name),
                    &get_current_date()
                ]
            )?;
        }
        if Database::is_table_empty("config", &con)? {
            con.execute(
                "INSERT INTO config (device, llm_model_path, gpu_layers, prompt_template) VALUES (?, ?, 20, ?)",
                &[
                    &Device::CPU as &dyn ToSql,
                    &"path/to/your/gguf/model.gguf",
                    &PromptTemplate::Default as &dyn ToSql
                ]
            )?;
        } 
        Ok(0)
    }

    pub fn is_table_empty(table_name: &str, con: &Connection) -> Result<bool> {
        let mut stmt = con.prepare(&format!("SELECT COUNT(*) FROM {}", table_name))?;
        let mut rows = stmt.query([])?;
        let count: i64 = rows.next()?.unwrap().get(0)?;
        Ok(count == 0)
    }

   /* pub fn get_messages() -> Result<Vec<Message>> {
        let con = Connection::open("companion_database.db")?;
        let mut stmt = con.prepare("SELECT id, ai, content, created_at FROM messages")?;
        let rows = stmt.query_map([], |row| {
            Ok(Message {
                id: row.get(0)?,
                ai: row.get(1)?,
                content: row.get(2)?,
                created_at: row.get(3)?,
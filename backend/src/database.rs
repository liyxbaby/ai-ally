
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
            })
        })?;
        let mut messages = Vec::new();
        for row in rows {
            messages.push(row?);
        }
        Ok(messages)
    } */

    pub fn get_x_messages(x: usize, index: usize) -> Result<Vec<Message>> {
        let con = Connection::open("companion_database.db")?;
        let mut stmt = con.prepare("SELECT id, ai, content, created_at FROM messages ORDER BY id DESC LIMIT ? OFFSET ?")?;
        let rows = stmt.query_map([x, index], |row| {
            Ok(Message {
                id: row.get(0)?,
                ai: row.get(1)?,
                content: row.get(2)?,
                created_at: row.get(3)?,
            })
        })?;
        let mut messages = Vec::new();
        for row in rows {
            messages.push(row?);
        }
        Ok(messages.into_iter().rev().collect())
    }

    pub fn get_latest_message() -> Result<Message> {
        let con = Connection::open("companion_database.db")?;
        let mut stmt = con.prepare("SELECT id, ai, content, created_at FROM messages ORDER BY id DESC LIMIT 1")?;
        let row = stmt.query_row([], |row| {
            Ok(Message {
                id: row.get(0)?,
                ai: row.get(1)?,
                content: row.get(2)?,
                created_at: row.get(3)?,
            })
        })?;
        Ok(row)
    }

    pub fn get_companion_data() -> Result<CompanionView> {
        let con = Connection::open("companion_database.db")?;
        let mut stmt = con.prepare("SELECT name, persona, example_dialogue, first_message, long_term_mem, short_term_mem, roleplay, dialogue_tuning, avatar_path FROM companion LIMIT 1")?;
        let row = stmt.query_row([], |row| {
            Ok(CompanionView {
                name: row.get(0)?,
                persona: row.get(1)?,
                example_dialogue: row.get(2)?,
                first_message: row.get(3)?,
                long_term_mem: row.get(4)?,
                short_term_mem: row.get(5)?,
                roleplay: row.get(6)?,
                dialogue_tuning: row.get(7)?,
                avatar_path: row.get(8)?,
            })
        })?;
        Ok(row)
    }

    pub fn get_companion_card_data() -> Result<CharacterCard> {
        let con = Connection::open("companion_database.db")?;
        let mut stmt = con.prepare("SELECT name, persona, first_message, example_dialogue FROM companion LIMIT 1")?;
        let row = stmt.query_row([], |row| {
            Ok(CharacterCard {
                name: row.get(0)?,
                description: row.get(1)?,
                first_mes: row.get(2)?,
                mes_example: row.get(3)?,
            })
        })?;
        Ok(row)
    }

    pub fn get_user_data() -> Result<UserView> {
        let con = Connection::open("companion_database.db")?;
        let mut stmt = con.prepare("SELECT name, persona FROM user LIMIT 1")?;
        let row: UserView = stmt.query_row([], |row| {
            Ok(UserView {
                name: row.get(0)?,
                persona: row.get(1)?,
            })
        })?;
        Ok(row)
    }

    pub fn get_message(id: i32) -> Result<Message> {
        let con = Connection::open("companion_database.db")?;
        let mut stmt = con.prepare("SELECT id, ai, content, created_at FROM messages WHERE id = ?")?;
        let row = stmt.query_row([id], |row| {
            Ok(Message {
                id: row.get(0)?,
                ai: row.get(1)?,
                content: row.get(2)?,
                created_at: row.get(3)?,
            })
        })?;
        Ok(row)
    }

    pub fn insert_message(message: NewMessage) -> Result<(), Error> {
        let con = Connection::open("companion_database.db")?;
        con.execute(
            &format!("INSERT INTO messages (ai, content, created_at) VALUES ({}, ?, ?)", message.ai),
            &[
                &message.content,
                &get_current_date()
            ]
        )?;
        Ok(())
    }

    pub fn edit_message(id: i32, message: NewMessage) -> Result<(), Error> {
        let con = Connection::open("companion_database.db")?;
        con.execute(
            &format!("UPDATE messages SET ai = {}, content = ? WHERE id = ?", message.ai),
            &[
                &message.content,
                &id.to_string()
            ]
        )?;
        Ok(())
    }

    pub fn delete_message(id: i32) -> Result<(), Error> {
        let con = Connection::open("companion_database.db")?;
        con.execute(
            "DELETE FROM messages WHERE id = ?",
            [id],
        )?;
        Ok(())
    }

    pub fn delete_latest_message() -> Result<(), rusqlite::Error> {
        let con = Connection::open("companion_database.db")?;
        let last_message_id: i32 = con.query_row(
            "SELECT id FROM messages ORDER BY id DESC LIMIT 1",
            [],
            |row| row.get(0)
        )?;
        con.execute(
            "DELETE FROM messages WHERE id = ?",
            [last_message_id]
        )?;
        Ok(())
    }

    pub fn erase_messages() -> Result<(), Error> {
        let con = Connection::open("companion_database.db")?;
        con.execute(
            "DELETE FROM messages",
            []
        )?;
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
        Ok(())
    }

    pub fn edit_companion(companion: CompanionView) -> Result<(), Error> {
        let con = Connection::open("companion_database.db")?;
        con.execute(
            &format!("UPDATE companion SET name = ?, persona = ?, example_dialogue = ?, first_message = ?, long_term_mem = {}, short_term_mem = {}, roleplay = {}, dialogue_tuning = {}, avatar_path = ?", companion.long_term_mem, companion.short_term_mem, companion.roleplay, companion.dialogue_tuning),
            &[
                &companion.name,
                &companion.persona,
                &companion.example_dialogue,
                &companion.first_message,
                &companion.avatar_path,
            ]
        )?;
        Ok(())
    }

    pub fn import_character_json(companion: CharacterCard) -> Result<(), Error> {
        let con = Connection::open("companion_database.db")?;
        con.execute(
            "UPDATE companion SET name = ?, persona = ?, example_dialogue = ?, first_message = ?",
            &[
                &companion.name,
                &companion.description,
                &companion.mes_example,
                &companion.first_mes
            ]
        )?;
        Ok(())
    }

    pub fn import_character_card(companion: CharacterCard, image_path: &str) -> Result<(), Error> {
        let con = Connection::open("companion_database.db")?;
        con.execute(
            "UPDATE companion SET name = ?, persona = ?, example_dialogue = ?, first_message = ?, avatar_path = ?",
            &[
                &companion.name,
                &companion.description,
                &companion.mes_example,
                &companion.first_mes,
                image_path
            ]
        )?;
        Ok(())
    }
        

    pub fn change_companion_avatar(avatar_path: &str) -> Result<(), Error> {
        let con = Connection::open("companion_database.db")?;
        con.execute(
            "UPDATE companion SET avatar_path = ?",
            &[
                avatar_path,
            ]
        )?;
        Ok(())
    }

    pub fn edit_user(user: UserView) -> Result<(), Error> {
        let con = Connection::open("companion_database.db")?;
        con.execute(
            "UPDATE user SET name = ?, persona = ?",
            &[
                &user.name,
                &user.persona,
            ]
        )?;
        Ok(())
    }

    pub fn get_config() -> Result<ConfigView> {
        let con = Connection::open("companion_database.db")?;
        let mut stmt = con.prepare("SELECT device, llm_model_path, gpu_layers, prompt_template FROM config LIMIT 1")?;
        let row = stmt.query_row([], |row| {
            Ok(ConfigView {
                device: row.get(0)?,
                llm_model_path: row.get(1)?,
                gpu_layers: row.get(2)?,
                prompt_template: row.get(3)?
            })
        })?;
        Ok(row)
    }

    pub fn change_config(config: ConfigModify) -> Result<(), Error> {
        let device = match config.device.as_str() {
            "CPU" => Device::CPU,
            "GPU" => Device::GPU,
            "Metal" => Device::Metal,
            _ => return Err(rusqlite::Error::InvalidParameterName("Invalid device type".to_string())),
        };
    
        let prompt_template = match config.prompt_template.as_str() {
            "Default" => PromptTemplate::Default,
            "Llama2" => PromptTemplate::Llama2,
            "Mistral" => PromptTemplate::Mistral,
            _ => return Err(rusqlite::Error::InvalidParameterName("Invalid prompt template type".to_string())),
        };
    
        let con = Connection::open("companion_database.db")?;
        con.execute(
            "UPDATE config SET device = ?, llm_model_path = ?, gpu_layers = ?, prompt_template = ?",
            &[
                &device as &dyn ToSql,
                &config.llm_model_path,
                &config.gpu_layers,
                &prompt_template as &dyn ToSql,
            ]
        )?;
        Ok(())
    }
}
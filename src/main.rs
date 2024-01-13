use std::fmt;
use std::fmt::Formatter;
use std::fs::OpenOptions;
use std::io::Write;
use std::time::SystemTime;
use chrono::{DateTime, Utc};
use json::*;

struct Cli {
    message: String,
}

struct BitaDbEntry {
    message: String,
    timestamp: String,
}

impl fmt::Display for BitaDbEntry {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} -- {}", self.message, self.timestamp)
    }
}

struct BitaDb {
    created_at: String,
    updated_at: String,
    messages: Vec<BitaDbEntry>,
}

fn main() {
    let message = std::env::args().nth(1).expect("no message :(");
    let args = Cli {
        message
    };
    write_to_bita_db(args);
    println!("Saved!");
}

fn write_to_bita_db(payload: Cli) {
    let now: String = get_now_timestamp();
    let entry = BitaDbEntry {
        message: payload.message,
        timestamp: now.to_string()
    };
    let db = BitaDb {
        created_at: now.to_string(),
        updated_at: now.to_string(),
        messages: Vec::from([entry])
    };

    //let file_exists = Path::exists(BITA_DB_LOCATION.as_ref());
    let file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(get_bita_db_path());
    let mut json_value = json::JsonValue::new_object();
    json_value["created_at"] = JsonValue::from(db.created_at);
    json_value["updated_at"] = JsonValue::from(db.updated_at);
    let mut messages = JsonValue::new_array();
    db.messages.iter().for_each(|message| {
        let mut item = JsonValue::new_object();
        item["message"] = JsonValue::from(message.message.to_string());
        item["timestamp"] = JsonValue::from(message.timestamp.to_string());
        messages.push(item).expect("Could not push item to messages db");
    });
    json_value["messages"] = messages;
    file.unwrap().write(json_value.dump().as_ref()).expect("Json bitadb could not be converted!");
}

fn get_now_timestamp() -> String {
    let now = SystemTime::now();
    let now: DateTime<Utc> = now.into();
    return now.to_rfc3339();
}

fn get_bita_db_path() -> String {
    match home::home_dir() {
        Some(path) => {
            //return path.display().to_string().as_ref() + "/.bita/bitadb.json";
            return format!("{}/.bita/bitadb.json", path.as_path().display().to_string());
        },
        None => {
            println!("Impossible to get your home dir!");
            panic!();
        },
    }
}

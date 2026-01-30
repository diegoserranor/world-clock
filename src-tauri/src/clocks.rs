use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{self, File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

const CLOCKS_FILE: &str = "clocks.json";

pub fn init() -> Result<Vec<Clock>, Box<dyn Error>> {
    let data_path = Path::new(CLOCKS_FILE);
    if !data_path.exists() {
        fs::write(data_path, "[]")?;
        return Ok(Vec::new());
    }
    let file = File::open(CLOCKS_FILE)?;
    let reader = BufReader::new(file);
    let mut clocks: Vec<Clock> = serde_json::from_reader(reader)?;
    clocks.sort_by_key(|clock| clock.order);
    let mut dirty = false;
    for (index, clock) in clocks.iter_mut().enumerate() {
        let expected = index as u32;
        if clock.order != expected {
            clock.order = expected;
            dirty = true;
        }
    }
    if dirty {
        save(&clocks)?;
    }
    Ok(clocks)
}

pub fn save(clocks: &Vec<Clock>) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(CLOCKS_FILE)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, clocks)?;
    Ok(())
}

// ---- clock ----

#[derive(Deserialize, Serialize, Clone)]
pub struct Clock {
    pub id: String,
    pub city_name: String,
    pub timezone: String,
    #[serde(default)]
    pub order: u32,
}

impl Clock {
    pub fn new(city_name: String, timezone: String, order: u32) -> Clock {
        let id = generate_unique_id();
        Clock {
            id,
            city_name,
            timezone,
            order,
        }
    }
}

/// Generate an ID based on a timestamp of the current time
fn generate_unique_id() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    format!("clock_{}", timestamp)
}

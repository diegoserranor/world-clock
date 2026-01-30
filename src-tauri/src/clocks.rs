use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{self, File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn init(path: &Path) -> Result<Vec<Clock>, Box<dyn Error>> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    if !path.exists() {
        fs::write(path, "[]")?;
        return Ok(Vec::new());
    }
    let file = File::open(path)?;
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
        save(path, &clocks)?;
    }
    Ok(clocks)
}

pub fn save(path: &Path, clocks: &Vec<Clock>) -> Result<(), Box<dyn Error>> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)?;
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

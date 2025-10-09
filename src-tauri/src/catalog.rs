use std::collections::{BTreeMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::time::Instant;
use serde::{Deserialize, Serialize};
use fst::{Map, IntoStreamer, Streamer};

pub fn init() -> Result<Catalog, Box<dyn Error>> {
    let now = Instant::now();

    let geonames = load_geonames()?;
    let cities = convert_geonames(geonames);
    let index = build_fst(&cities)?;

    println!("init catalog in {:.2}ms", now.elapsed().as_millis());

    Ok(Catalog { cities, index })
}

fn build_fst(cities: &Vec<City>) -> Result<Map<Vec<u8>>, Box<dyn Error>> {
    let now = Instant::now();

    // Create a BTreeMap
    // This collects all city names and their geoname IDs
    let mut btree = BTreeMap::new();
    for city in cities {
        // Add primary name
        btree.insert(city.name.to_lowercase(), city.geonameid as u64);

        // Add ASCII name if different from primary name
        if city.asciiname != city.name {
            btree.insert(city.asciiname.to_lowercase(), city.geonameid as u64);
        }

        // Add alternate names
        if !city.alternatenames.is_empty() {
            for alt_name in city.alternatenames.split(',') {
                let alt_name = alt_name.trim();
                if !alt_name.is_empty() && alt_name != city.name && alt_name != city.asciiname {
                    btree.insert(alt_name.to_lowercase(), city.geonameid as u64);
                }
            }
        }
    }

    println!(
        "created btree with {} mappings in {:.2}ms",
        btree.len(),
        now.elapsed().as_millis()
    );

    let now = Instant::now();
    let mut builder = fst::MapBuilder::memory();
    for (name, id) in btree {
        builder.insert(name, id)?;
    }
    let fst = builder.into_map();

    println!(
        "created fst with {} mappings in {:.2}ms",
        fst.len(),
        now.elapsed().as_millis()
    );

    Ok(fst)
}

fn convert_geonames(geonames: Vec<Geoname>) -> Vec<City> {
    let mut cities: Vec<City> = Vec::new();
    for geoname in geonames {
        cities.push(from_geoname(geoname));
    }
    cities
}

// ---- catalog ----

pub struct Catalog {
    cities: Vec<City>,
    index: fst::Map<Vec<u8>>,
}

impl Catalog {
    pub fn search(&self, prefix: &str) -> Vec<City> {
        let prefix = prefix.to_lowercase();
        let mut results: Vec<u64> = Vec::new();
        let mut seen_ids = HashSet::new();

        // Create a stream for keys that start with the prefix
        let mut stream = self.index.range().ge(&prefix).into_stream();

        while let Some((key, value)) = stream.next() {
            let key = String::from_utf8_lossy(&key);
            if key.starts_with(&prefix) {
                if seen_ids.insert(value) {
                    results.push(value);

                    // Limit results to avoid overwhelming the user
                    if results.len() >= 100 {
                        break;
                    }
                }
            } else {
                // Since the FST is sorted, we can break early when we pass the prefix
                break;
            }
        }

        let mut cities: Vec<City> = Vec::new();
        for result in results.into_iter() {
            match self.find_by_id(result) {
                Some(city) => cities.push(city.clone()),
                None => continue,
            }
        }
        cities
    }

    pub fn find_by_id(&self, id: u64) -> Option<&City> {
        self.cities.iter().find(|city| city.geonameid == id)
    }
}

// ---- cities ----

#[derive(Deserialize, Serialize, Clone)]
pub struct City {
    pub geonameid: u64,
    pub name: String,
    pub asciiname: String,
    pub alternatenames: String,
    pub country_code: String,
    pub timezone: String,
}

fn from_geoname(geoname: Geoname) -> City {
    City {
        geonameid: geoname.geonameid,
        name: geoname.name,
        asciiname: geoname.asciiname,
        alternatenames: geoname.alternatenames,
        country_code: geoname.country_code,
        timezone: geoname.timezone,
    }
}

// ----- geonames ----

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct Geoname {
    pub geonameid: u64,
    pub name: String,
    pub asciiname: String,
    pub alternatenames: String,
    pub latitude: f64,
    pub longitude: f64,
    pub feature_class: String,
    pub feature_code: String,
    pub country_code: String,
    pub cc2: String,
    pub admin1_code: String,
    pub admin2_code: String,
    pub admin3_code: String,
    pub admin4_code: String,
    pub population: Option<i64>,
    pub elevation: Option<i32>,
    pub dem: Option<i32>,
    pub timezone: String,
    pub modification_date: String,
}

const EXPECTED_TSV_FIELDS: usize = 19;

fn load_geonames() -> Result<Vec<Geoname>, Box<dyn Error>> {
    let start_time = Instant::now();
    println!("starting to load cities from data/cities1000.txt...");

    let file_path = Path::new("data/cities1000.txt");
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut cities = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let fields: Vec<&str> = line.split('\t').collect();

        // Skip malformed lines
        if fields.len() < EXPECTED_TSV_FIELDS {
            continue;
        }

        let city = Geoname {
            geonameid: fields[0].parse().unwrap_or(0),
            name: fields[1].to_string(),
            asciiname: fields[2].to_string(),
            alternatenames: fields[3].to_string(),
            latitude: fields[4].parse().unwrap_or(0.0),
            longitude: fields[5].parse().unwrap_or(0.0),
            feature_class: fields[6].to_string(),
            feature_code: fields[7].to_string(),
            country_code: fields[8].to_string(),
            cc2: fields[9].to_string(),
            admin1_code: fields[10].to_string(),
            admin2_code: fields[11].to_string(),
            admin3_code: fields[12].to_string(),
            admin4_code: fields[13].to_string(),
            population: fields[14].parse().ok(),
            elevation: fields[15].parse().ok(),
            dem: fields[16].parse().ok(),
            timezone: fields[17].to_string(),
            modification_date: fields[18].to_string(),
        };

        cities.push(city);
    }

    let load_duration = start_time.elapsed();
    println!(
        "loaded {} cities in {:.2}ms",
        cities.len(),
        load_duration.as_millis()
    );

    Ok(cities)
}


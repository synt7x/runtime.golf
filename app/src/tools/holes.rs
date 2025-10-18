use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, sync::OnceLock};

#[derive(Debug, Serialize, Deserialize)]
pub struct Hole {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub kind: String,
    pub instructions: String,
}

static HOLES: OnceLock<HashMap<String, Hole>> = OnceLock::new();

pub fn load() {
    let mut holes = HashMap::new();
    println!("Loading holes from ../holes");
    for entry in fs::read_dir("../holes").expect("Failed to load holes") {
        let directory = entry.expect("Failed to read hole").path();
        if !directory.is_dir() {
            continue;
        }

        let path = directory.join("hole.toml");
        if path.extension().and_then(|s| s.to_str()) == Some("toml") {
            let content = fs::read_to_string(&path).expect("Failed to read hole file");
            let hole: Hole = toml::from_str(&content).expect("Failed to parse hole file");
            println!("Loaded hole {} from {:?}", hole.title, path);

            holes.insert(hole.title.clone(), hole);
        }
    }
    println!("Successfully loaded {} holes", holes.len());
    HOLES.set(holes).expect("Holes map already constructed");
}

pub fn get() -> &'static HashMap<String, Hole> {
    return HOLES.get().expect("Holes map not constructed");
}

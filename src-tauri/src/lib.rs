mod catalog;
mod clocks;

use std::sync::RwLock;
use tauri::{Manager, State};

// ---- app state ----

pub struct AppState {
    pub catalog: catalog::Catalog,
    pub clocks: RwLock<Vec<clocks::Clock>>,
}

// ---- commands ----

/// Retrieves the saved clocks
#[tauri::command]
fn get_clocks(state: State<'_, AppState>) -> Result<Vec<clocks::Clock>, String> {
    let clocks = state.clocks.read().map_err(|e| e.to_string())?;
    Ok(clocks.clone())
}

/// Adds a clock to the list of saved clocks
#[tauri::command(rename_all = "snake_case")]
fn add_clock(
    state: State<'_, AppState>,
    city_name: String,
    timezone: String,
) -> Result<(), String> {
    let clock = clocks::Clock::new(city_name, timezone);

    let mut clocks = state.clocks.write().map_err(|e| e.to_string())?;
    clocks.push(clock);

    let persisted = clocks.clone();
    drop(clocks);

    clocks::save(&persisted).map_err(|e| format!("Failed to save clocks: {}", e))?;
    Ok(())
}

/// Deletes a clock from the list of saved clocks
#[tauri::command(rename_all = "snake_case")]
fn delete_clock(state: State<'_, AppState>, clock_id: String) -> Result<bool, String> {
    let mut clocks = state.clocks.write().map_err(|e| e.to_string())?;
    if let Some(position) = clocks.iter().position(|clock| clock.id == clock_id) {
        clocks.remove(position);
        let persisted = clocks.clone();
        drop(clocks);
        clocks::save(&persisted).map_err(|e| format!("Failed to save updated clocks: {}", e))?;
        Ok(true)
    } else {
        Ok(false)
    }
}

/// Search for cities from the catalog with a string prefix (search word for the FST)
#[tauri::command]
fn search_cities(state: State<'_, AppState>, prefix: String) -> Result<Vec<catalog::City>, String> {
    Ok(state.catalog.search(&prefix))
}

// Get city information by geoname ID
#[tauri::command]
fn get_city_by_id(state: State<'_, AppState>, id: u64) -> Result<catalog::City, String> {
    match state.catalog.find_by_id(id) {
        Some(city) => return Ok(city.clone()),
        None => return Err("no city found".to_string()),
    }
}

/// Run the Tauri app
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let catalog = catalog::init().expect("failed to init catalog");
    let clocks = clocks::init().expect("failed to init clocks");
    let app_state = AppState {
        catalog,
        clocks: RwLock::new(clocks),
    };
    tauri::Builder::default()
        .setup(|app| {
            app.manage(app_state);
            Ok(())
        })
        .plugin(tauri_plugin_opener::init()) // Heap: Plugin state allocated on heap
        .invoke_handler(tauri::generate_handler![
            get_clocks,
            add_clock,
            delete_clock,
            search_cities,
            get_city_by_id
        ])
        .run(tauri::generate_context!()) // Heap: App context and state on heap
        .expect("error while running tauri application"); // Error handling: panic on failure
}


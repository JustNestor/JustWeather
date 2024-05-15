#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use minreq;
use serde_json;

mod airindex;
mod citysearch;
mod weather;

const API_KEY: &str = "21813acf1347c5dab9268421b7709117";

#[tauri::command]
fn search_city(name: &str) -> String {
    let response = citysearch::CitySearch::search(API_KEY, name);
    let json: String = serde_json::to_string(&response).unwrap().replace('\"', "'");

    println!("{:?}", json);

    json
}

#[tauri::command]
fn get_weather(lat: String, lon: String) -> String {
    let response = weather::WeatherResponse::get_response(API_KEY, lat, lon);
    let json: String = serde_json::to_string(&response).unwrap().replace('\"', "'");

    println!("{:?}", json);

    json
}

#[tauri::command]
fn get_airindex(lat: String, lon: String) -> String {
    let response = airindex::AirIndex::get_airindex(API_KEY, lat, lon);
    let json: String = serde_json::to_string(&response).unwrap().replace('\"', "'");

    println!("{:?}", json);

    json
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            search_city,
            get_weather,
            get_airindex
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

pub mod earth;
pub mod planes;
use earth::EarthPlugin;
use planes::PlanesPlugin;
use wasm_bindgen::prelude::*;

use serde::{Deserialize, Serialize};

use bevy::prelude::*;

pub fn run() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EarthPlugin)
        .add_plugin(PlanesPlugin)
        .run();
}

#[wasm_bindgen(module = "/out/get_flight.js")]
extern "C" {
    fn get_flight() -> String;
}

#[derive(Serialize, Deserialize)]
pub struct FlightInfo {
    pub uuid: String,
    pub latitude: f32,
    pub longitude: f32,
    pub landed: bool,
}

pub fn get_flights_from_js() -> Option<FlightInfo> {
    let data: String = get_flight();

    let pieces: Vec<&str> = data.split(';').collect();

    if pieces.len() == 4 {
        // Extract individual values and perform type conversion
        let uuid = pieces[0];
        let longitude: f32 = pieces[1].parse().unwrap();
        let latitude: f32 = pieces[2].parse().unwrap();
        let landed: bool = pieces[3].parse().unwrap();

        return Some(FlightInfo {
            uuid: uuid.to_string(),
            latitude: latitude,
            longitude: longitude,
            landed: landed,
        });
    }

    return None;
}

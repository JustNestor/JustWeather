use minreq;
use serde::{Deserialize, Serialize};
use serde_json;
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct CitySearch {
    result: Vec<City>,
}

#[derive(Debug, Serialize, Deserialize)]
struct City {
    name: String,
    state: String,
    lat: String,
    lon: String,
    country: String,
}

impl CitySearch {
    pub fn search(api_key: &str, name: &str) -> Self {
        let mut cities = Vec::new();

        let geo_api = format!(
            "http://api.openweathermap.org/geo/1.0/direct?q={name}&limit=10&appid={api_key}"
        );

        let binding = minreq::get(geo_api).with_timeout(10).send().unwrap();
        let resp = binding.as_str().unwrap();

        let json: Value = serde_json::from_str(resp).unwrap();

        if let Some(array) = json.as_array() {
            for city in array {
                let mut city_country = city["country"].as_str().unwrap_or_default().to_string();

                if city_country == "UA".to_owned() {
                    city_country = "Україна".to_owned();
                }

                let c = City {
                    name: city["local_names"]["uk"]
                        .as_str()
                        .unwrap_or(&city["name"].to_string())
                        .to_string(),
                    state: city["state"].as_str().unwrap_or_default().to_string(),
                    lat: city["lat"].to_string(),
                    lon: city["lon"].to_string(),
                    country: city_country,
                };

                cities.push(c);
            }
        }

        CitySearch { result: cities }
    }
}

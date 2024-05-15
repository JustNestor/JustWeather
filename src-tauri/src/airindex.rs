use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct AirIndex {
    pm25: String,
    so2: String,
    no2: String,
    o3: String,
    status: String,
}

impl AirIndex {
    pub fn get_airindex(api: &str, lat: String, lon: String) -> Self {
        let mut airindex = AirIndex {
            pm25: "".to_owned(),
            so2: "".to_owned(),
            no2: "".to_owned(),
            o3: "".to_owned(),
            status: "".to_owned(),
        };

        let air_api = format!(
            "https://api.openweathermap.org/data/2.5/air_pollution?lat={lat}&lon={lon}&appid={api}"
        );

        let binding = minreq::get(air_api).with_timeout(10).send().unwrap();
        let resp = binding.as_str().unwrap();

        let json: Value = serde_json::from_str(resp).unwrap();

        if let Some(array) = json["list"].as_array() {
            for i in array {
                let binding = i["main"]["aqi"].to_string();
                let aqi = binding.as_str();

                let aqi = match aqi {
                    "1" => "Дуже добре",
                    "2" => "Добре",
                    "3" => "Середнє",
                    "4" => "Погано",
                    "5" => "Дуже погано",
                    _ => "Не визначено",
                };

                let aqi = aqi.to_string();
                let cmp = &i["components"];

                airindex = AirIndex {
                    pm25: cmp["pm2_5"].to_string(),
                    so2: cmp["so2"].to_string(),
                    no2: cmp["no2"].to_string(),
                    o3: cmp["o3"].to_string(),
                    status: aqi,
                };
            }
        }

        airindex
    }
}

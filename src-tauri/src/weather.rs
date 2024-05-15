use serde::{Deserialize, Serialize};
use serde_json::Value;

use chrono::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct WeatherResponse {
    now: WeatherNow,
    hourlyforecast: HourlyForecast,
    fivedayforecast: FiveDayForecast,
}

#[derive(Debug, Serialize, Deserialize)]
struct WeatherNow {
    date: Vec<String>,
    weather_name: String,
    icon: String,
    temp: f64,
    feelslike: f64,
    pressure: u64,
    humidity: u64,
    visibility: u64,
    sunrise: String,
    sunset: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct HourlyForecast {
    forecast: Vec<Hourly>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Hourly {
    time: String,
    icon: String,
    temp: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct FiveDayForecast {
    forecast: Vec<DayForecast>,
}

#[derive(Debug, Serialize, Deserialize)]
struct DayForecast {
    date: Vec<String>,
    icon: String,
    temp: f64,
}

impl WeatherResponse {
    pub fn get_response(api_key: &str, lat: String, lon: String) -> Self {
        let json = Self::parse_weather(api_key, &lat, &lon);

        let weather_now = Self::get_weather_now(&json);
        let weather_hourly = Self::get_weather_hourly(&json);
        let weather_daily = Self::get_weather_daily(&json);

        Self {
            now: weather_now,
            hourlyforecast: weather_hourly,
            fivedayforecast: weather_daily,
        }
    }

    fn get_weather_daily(json: &Value) -> FiveDayForecast {
        #[derive(Debug, Clone)]
        struct Day {
            date: Vec<String>,
            temps: Vec<f64>,
            icons: Vec<String>,
        }

        impl Day {
            pub fn get_temp(&self) -> f64 {
                *self
                    .temps
                    .iter()
                    .max_by(|a, b| a.total_cmp(b))
                    .unwrap_or(&0.0)
            }

            pub fn get_icon<'a>(&self) -> String {
                let mut day_icons: HashMap<&str, usize> = HashMap::new();

                for icon in self.icons.iter() {
                    if icon.contains("d") {
                        *day_icons.entry(icon).or_insert(0) += 1;
                    };
                }

                let icon = day_icons
                    .iter()
                    .max_by_key(|&(_, count)| count)
                    .unwrap_or_else(|| (&"", &0))
                    .0;

                icon.to_string()
            }
        }

        let mut hash: HashMap<String, Day> = HashMap::new();
        let mut forecast: Vec<DayForecast> = Vec::new();

        if let Some(array) = json["list"].as_array() {
            for weather in array.iter() {
                let binding = weather["dt_txt"].to_string();
                let date = binding
                    .split_whitespace()
                    .next()
                    .unwrap_or_default()
                    .replace("\"", "");
                let date_vec = Self::format_date(&date);

                let temp = weather["main"]["temp"].as_f64().unwrap_or(0.0);
                let icon = weather["weather"][0]["icon"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string();

                hash.entry(date)
                    .and_modify(|e| {
                        e.icons.push(icon.to_owned());
                        e.temps.push(temp);
                    })
                    .or_insert(Day {
                        date: date_vec,
                        temps: vec![temp],
                        icons: vec![icon],
                    });
            }

            let mut sorted_vec: Vec<(String, Day)> = hash.clone().into_iter().collect();
            sorted_vec.sort_by(|(date1, _), (date2, _)| date1.cmp(date2));

            for (_, day) in sorted_vec.iter().skip(1) {
                let temp = day.get_temp();
                let icon = day.get_icon();
                let date = day.date.clone();

                forecast.push(DayForecast { date, icon, temp });
            }
        }

        FiveDayForecast { forecast: forecast }
    }

    fn get_weather_hourly(json: &Value) -> HourlyForecast {
        let mut forecast: Vec<Hourly> = Vec::new();

        if let Some(array) = json["list"].as_array() {
            for weather in array {
                let binding = weather["dt_txt"].to_string();
                let time = binding
                    .split_whitespace()
                    .next_back()
                    .unwrap_or_default()
                    .replace("\"", "");

                let hour = time.splitn(2, ":").next().unwrap_or_default();
                let time = format!("{}:00", hour);

                let icon = weather["weather"][0]["icon"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string();
                let temp = weather["main"]["temp"].as_f64().unwrap_or(0.0);

                let hourly = Hourly { time, icon, temp };
                forecast.push(hourly);
            }
        } else {
            panic!("Bad json!");
        }

        HourlyForecast {
            forecast: forecast[..8].to_vec(),
        }
    }

    fn get_weather_now(json: &Value) -> WeatherNow {
        let weather: WeatherNow;

        if let Some(array) = json["list"].as_array() {
            let weather_today = &array[0];

            let binding = weather_today["dt_txt"].to_string();
            let date_str = binding
                .split_whitespace()
                .next()
                .unwrap_or_default()
                .replace("\"", "");

            let date = Self::format_date(&date_str);

            let sunrise_unix = json["city"]["sunrise"].to_string();
            let sunset_unix = json["city"]["sunset"].to_string();

            let sunrise = Self::unix_to_utc_time(sunrise_unix);
            let sunset = Self::unix_to_utc_time(sunset_unix);

            let weather_name_english = weather_today["weather"][0]["main"]
                                .as_str()
                                .unwrap_or_default()
                                .to_string();

           	let weather_name = Self::translate_weather(weather_name_english);

            weather = WeatherNow {
                date,
                weather_name,
                icon: weather_today["weather"][0]["icon"]
                    .as_str()
                    .unwrap_or_default()
                    .to_string(),
                temp: weather_today["main"]["temp"].as_f64().unwrap_or(0.0),
                feelslike: weather_today["main"]["feels_like"].as_f64().unwrap_or(0.0),
                pressure: weather_today["main"]["pressure"].as_u64().unwrap_or(0),
                humidity: weather_today["main"]["humidity"].as_u64().unwrap_or(0),
                visibility: weather_today["visibility"].as_u64().unwrap_or(0) / 1000,
                sunrise,
                sunset,
            };
        } else {
            panic!("Bad json!");
        }

        weather
    }

    fn parse_weather(api_key: &str, lat: &str, lon: &str) -> Value {
        let forecast_api = format!(
            "https://api.openweathermap.org/data/2.5/forecast?lat={lat}&lon={lon}&units=metric&appid={api_key}"
        );

        let binding = minreq::get(forecast_api)
            .with_timeout(10)
            .send()
            .expect("Can't get response!");
        let resp = binding.as_str().unwrap_or_default();

        serde_json::from_str(resp).unwrap_or_default()
    }

    fn format_date(date: &str) -> Vec<String> {
        let days_of_week_short: [&str; 7] = ["Пн", "Вт", "Ср", "Чт", "Пт", "Сб", "Нд"];

        let days_of_week_full: [&str; 7] = [
            "Понеділок",
            "Вівторок",
            "Середа",
            "Четвер",
            "П`ятниця",
            "Субота",
            "Неділя",
        ];

        let months_short: [&str; 12] = [
            "Січ", "Лют", "Бер", "Кві", "Тра", "Чер", "Лип", "Сер", "Вер", "Жов", "Лис", "Гру",
        ];

        let parsed_date = NaiveDate::parse_from_str(date, "%Y-%m-%d").unwrap_or_default();

        let day: usize = parsed_date
            .weekday()
            .num_days_from_monday()
            .try_into()
            .unwrap_or_default();
        let month: usize = parsed_date.month0().try_into().unwrap_or_default();
        let numeric_day = parsed_date.day0() + 1;

        vec![
            numeric_day.to_string(),
            days_of_week_short[day].to_string(),
            days_of_week_full[day].to_string(),
            months_short[month].to_string(),
        ]
    }

    fn unix_to_utc_time(timestamp: String) -> String {
        let timestamp: i64 = timestamp.parse().unwrap_or_default();
        let timestamp = timestamp + 10800;

        let datetime = DateTime::from_timestamp(timestamp, 0).unwrap_or_default();
        let newdate = datetime.format("%H:%M").to_string();

        newdate
    }

    fn translate_weather(weather: String) -> String {
    	let translations = HashMap::from([
    		("Clear", "Ясно"),
    		("Clouds", "Хмарно"),
    		("Rain", "Дощ"),
    		("Thunderstorm", "Гроза"),
    		("Snow", "Сніг"),
    		("Mist", "Туман")
    	]);

    	match translations.get(weather.as_str()) {
    		Some(translation) => translation.to_string(),
    		_ => weather,
    	}
    }
}

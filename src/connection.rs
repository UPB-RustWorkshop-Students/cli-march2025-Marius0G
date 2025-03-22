use crate::app::{City, AppResult};
use chrono::{DateTime, Local, TimeZone, Utc}; // Add Utc import here
use reqwest;
use serde_json::Value;

pub struct Connection {
    api_key: String,
}

impl Connection {
    pub fn new() -> AppResult<Self> {
        Ok(Self {
            api_key: "8c1e6c9d40c2c471f544e4bd99757965".to_string(),
        })
    }

    /// Method that is handling the request to the OpenWeather api
    /// and parsing the response
    ///
    /// Returns weather details about a certain city
    pub async fn get_data(&self, city: &str) -> AppResult<City> {
        let url = format!(
            "http://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric", 
            city, 
            self.api_key
        );
        
        let response = reqwest::get(&url).await?.text().await?;
        let weather_data: Value = serde_json::from_str(&response)?;
        
        // Extract all weather details from the JSON
        let temp = weather_data["main"]["temp"].as_f64().unwrap_or(0.0) as f32;
        let feels_like = weather_data["main"]["feels_like"].as_f64().unwrap_or(0.0) as f32;
        let temp_min = weather_data["main"]["temp_min"].as_f64().unwrap_or(0.0) as f32;
        let temp_max = weather_data["main"]["temp_max"].as_f64().unwrap_or(0.0) as f32;
        let humidity = weather_data["main"]["humidity"].as_u64().unwrap_or(0) as u8;
        let pressure = weather_data["main"]["pressure"].as_u64().unwrap_or(0) as u32;
        
        // Wind data
        let wind_speed = weather_data["wind"]["speed"].as_f64().unwrap_or(0.0) as f32;
        let wind_direction = weather_data["wind"]["deg"].as_u64().unwrap_or(0) as u16;
        
        // Weather description
        let weather_main = weather_data["weather"][0]["main"].as_str().unwrap_or("Unknown").to_string();
        let weather_description = weather_data["weather"][0]["description"].as_str().unwrap_or("Unknown").to_string();
        let weather_icon = weather_data["weather"][0]["icon"].as_str().unwrap_or("01d").to_string();
        
        // Additional data
        let visibility = weather_data["visibility"].as_u64().unwrap_or(0) as u32;
        let cloudiness = weather_data["clouds"]["all"].as_u64().unwrap_or(0) as u8;
        let country = weather_data["sys"]["country"].as_str().unwrap_or("??").to_string();
        
        // Time data - convert Unix timestamps to DateTime
        let sunrise_ts = weather_data["sys"]["sunrise"].as_i64().unwrap_or(0);
        let sunset_ts = weather_data["sys"]["sunset"].as_i64().unwrap_or(0);
        let timezone_offset = weather_data["timezone"].as_i64().unwrap_or(0);
        
        let sunrise = Utc.timestamp_opt(sunrise_ts, 0).unwrap().with_timezone(&Local);
        let sunset = Utc.timestamp_opt(sunset_ts, 0).unwrap().with_timezone(&Local);
        
        Ok(City {
            name: city.to_string(),
            selected: false,
            temperature: temp,
            feels_like,
            temp_min,
            temp_max,
            humidity,
            pressure,
            wind_speed,
            wind_direction,
            weather_main,
            weather_description,
            weather_icon,
            visibility,
            cloudiness,
            country,
            sunrise,
            sunset,
        })
    }
    
    pub async fn get_cities(&self) -> AppResult<Vec<City>> {
        let city_names = vec!["Bucharest", "Suceava", "Galati"];
        let mut cities = Vec::new();
        
        for (i, name) in city_names.iter().enumerate() {
            let mut city = self.get_data(name).await?;
            // First city is selected by default
            city.selected = i == 0;
            cities.push(city);
        }
        
        Ok(cities)
    }
}
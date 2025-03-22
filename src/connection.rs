use crate::app::{City, AppResult};
use chrono::{DateTime, Local};
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
        
        // Extract weather details from the JSON
        let temp = weather_data["main"]["temp"].as_f64().unwrap_or(0.0) as f32;
        let humidity = weather_data["main"]["humidity"].as_u64().unwrap_or(0) as u8;
        let wind_speed = weather_data["wind"]["speed"].as_f64().unwrap_or(0.0) as f32;
        
        Ok(City {
            name: city.to_string(),
            selected: false,
            temperature: temp,
            humidity,
            wind_speed,
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
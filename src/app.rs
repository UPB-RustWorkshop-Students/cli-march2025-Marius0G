use std::error;
use std::fs::OpenOptions;
use std::io::Write;
use tokio::runtime::Runtime;
use crate::connection::Connection;
use chrono::{DateTime, Local, TimeZone, Utc};
/// Application result type.
pub type AppResult<T> = Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub struct City {
    // Base data
    pub name: String,
    pub selected: bool,
    
    // Main weather data
    pub temperature: f32,
    pub feels_like: f32,
    pub temp_min: f32,
    pub temp_max: f32,
    pub humidity: u8,
    pub pressure: u32,
    
    // Wind data
    pub wind_speed: f32,
    pub wind_direction: u16,
    
    // Weather description
    pub weather_main: String,
    pub weather_description: String,
    pub weather_icon: String,
    
    // Additional data
    pub visibility: u32,
    pub cloudiness: u8,
    pub country: String,
    pub sunrise: DateTime<Local>,
    pub sunset: DateTime<Local>,
}
impl City {
    // Helper method to get weather icon URL
    pub fn icon_url(&self) -> String {
        format!("https://openweathermap.org/img/wn/{}@2x.png", self.weather_icon)
    }
    
    // Helper to get formatted time
    pub fn sunrise_formatted(&self) -> String {
        self.sunrise.format("%H:%M").to_string()
    }
    
    pub fn sunset_formatted(&self) -> String {
        self.sunset.format("%H:%M").to_string()
    }
}
/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub cities: Vec<City>,
    pub running: bool,
}

impl App {

    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        let conn_result = Connection::new();
        let cities = match conn_result {
            Ok(conn) => {
                // WRONG: Creating a new runtime when one already exists
                // let rt = Runtime::new().unwrap();
                // match rt.block_on(conn.get_cities()) { ... }
                
                // Instead, use a blocking operation in the current runtime:
                match tokio::task::block_in_place(|| {
                    tokio::runtime::Handle::current().block_on(conn.get_cities())
                }) {
                    Ok(cities) => cities,
                    Err(_) => {
                        // Fallback with empty cities vector on error
                        Vec::new()
                    }
                }
            },
            Err(_) => {
                // Fallback with empty cities vector if connection fails
                Vec::new()
            }
        };
        Self {
            running: true,
            cities,
        }
    }
    pub fn next_city(&mut self) {
        if self.cities.is_empty() {
            return;
        }
        
        // Find the currently selected city index
        let current_index = self.cities.iter().position(|city| city.selected).unwrap_or(0);
        
        // Unselect the current city
        if let Some(city) = self.cities.get_mut(current_index) {
            city.selected = false;
        }
        
        // Select the next city (with wraparound)
        let next_index = (current_index + 1) % self.cities.len();
        if let Some(city) = self.cities.get_mut(next_index) {
            city.selected = true;
        }
    }
    
    
    pub fn prev_city(&mut self) {
        if self.cities.is_empty() {
            return;
        }
        
        // Find the currently selected city index
        let current_index = self.cities.iter().position(|city| city.selected).unwrap_or(0);
        
        // Unselect the current city
        if let Some(city) = self.cities.get_mut(current_index) {
            city.selected = false;
        }
        
        // Select the previous city (with wraparound)
        let prev_index = if current_index > 0 {
            current_index - 1
        } else {
            self.cities.len() - 1
        };
        
        if let Some(city) = self.cities.get_mut(prev_index) {
            city.selected = true;
        }
    }
}

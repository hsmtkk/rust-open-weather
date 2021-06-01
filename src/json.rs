use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct WeatherData {
    pub weather: Vec<Weather>,
    pub main: Main,
}

#[derive(Serialize, Deserialize)]
pub struct Weather {
    pub main: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
pub struct Main {
    pub temp: f64,
    pub humidity: f64,
}

pub fn parse_json(json: &str) -> Result<WeatherData> {
    let w: WeatherData = serde_json::from_str(json)?;
    return Ok(w);
}

#[test]
fn test_parse_json(){
    let json = std::fs::read_to_string("./src/example.json").expect("failed to read file");
    let data = parse_json(&json).expect("failed to parse json");
    assert_eq!("Clouds", data.weather[0].main);
    assert_eq!("few clouds", data.weather[0].description);
    assert_eq!(297.64, data.main.temp);
    assert_eq!(54.0, data.main.humidity);
}
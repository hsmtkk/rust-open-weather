use std::env;
use std::process;

use anyhow::Result;

mod json;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: rust-open-weather state");
        process::exit(1);
    }
    let state = &args[1];
    let api_key = get_api_key();
    get_data(&state, &api_key).expect("failed to get data");
}

const OPEN_WEATHER_API_KEY: &str = "OPEN_WEATHER_API_KEY";

fn get_api_key() -> String {
    let val = env::var(OPEN_WEATHER_API_KEY).expect(&format!("environment variable {} is not defined", OPEN_WEATHER_API_KEY)); 
    return val;
}

fn get_data(state:&str, api_key: &str) -> Result<()> {
    let url = format!("https://api.openweathermap.org/data/2.5/weather?q={}&appid={}", state, api_key);
    let body = reqwest::blocking::get(url)?.text()?;
    let data = json::parse_json(&body)?;
    println!("Main: {}", data.weather[0].main);
    println!("Descrption: {}", data.weather[0].description);
    println!("Temperature: {}", data.main.temp - 273.15);
    println!("Humidity: {}", data.main.humidity);
    return Ok(());
}
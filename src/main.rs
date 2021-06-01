use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: rust-open-weather state");
        process::exit(1);
    }
    let state = &args[1];
    println!("state = {}", &state);
    let key = get_api_key();
    println!("key = {}", &key);
}

const OPEN_WEATHER_API_KEY: &str = "OPEN_WEATHER_API_KEY";

fn get_api_key() -> String {
    let val = env::var(OPEN_WEATHER_API_KEY).expect(&format!("environment variable {} is not defined", OPEN_WEATHER_API_KEY)); 
    return val;
}
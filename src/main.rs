use chrono::Utc;
use serde_json::{json, to_string_pretty, Map, Value};
use std::error::Error;
use std::fs;
use std::thread::sleep;
use std::time::Duration;

const DAYS_PATH: &str = "/home/matiasop98/tmp/days.json";
const MONTHS_PATH: &str = "/home/matiasop98/tmp/months.json";
const MINUTES_STEP: i64 = 1;

fn main() {
    let step = Duration::new(60 * (MINUTES_STEP as u64), 0);

    loop {
        sleep(step);
        let utc = Utc::now();

        // Read Files
        let mut days = read_file(DAYS_PATH).unwrap();
        let mut months = read_file(MONTHS_PATH).unwrap();

        // Get current date and month
        let day_current = utc.date().format("%Y-%m-%d").to_string();
        let month_current = utc.date().format("%Y-%m").to_string();

        // If this is a new day or month, create a new Map entry with value MINUTES_STEP
        // Else, add MINUTES_STEP to days and month
        days = check_and_insert(days, day_current);
        months = check_and_insert(months, month_current);

        // Write days and months to file
        fs::write(DAYS_PATH, to_string_pretty(&days).unwrap()).expect("Unable to write to file");
        fs::write(MONTHS_PATH, to_string_pretty(&months).unwrap())
            .expect("Unable to write to file");
    }
}

fn check_and_insert(mut hashmap: Map<String, Value>, current: String) -> Map<String, Value> {
    if hashmap.contains_key(&current) {
        let aux = hashmap.get(&current).unwrap().as_i64().unwrap();
        hashmap.insert(current, json!(aux + MINUTES_STEP));
    } else {
        hashmap.insert(current, json!(MINUTES_STEP));
    }
    hashmap
}

fn read_file(path: &str) -> Result<Map<String, Value>, Box<dyn Error>> {
    let data = fs::read_to_string(path)?;
    let parsed: Value = serde_json::from_str(&data)?;
    let obj: Map<String, Value> = parsed.as_object().unwrap().clone();
    Ok(obj)
}

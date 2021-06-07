use chrono::{Datelike, Utc};
use csv::Writer;
use serde_json::{json, to_string_pretty, Map, Value};
use std::env;
use std::error::Error;
use std::fs;
use std::thread::sleep;
use std::time::Duration;

const MINUTES_STEP: i64 = 1;

fn main() {
    // json_to_csv("days.json", "days.csv");
    // add_json("data/j1.json", "data/j2.json", "data/result.json");
    let step = Duration::new(60 * (MINUTES_STEP as u64), 0);

    // Get paths
    let args: Vec<String> = env::args().collect();
    let days_path: &str = &args[1];
    let weeks_path: &str = &args[2];
    let months_path: &str = &args[3];
    println!("{}", days_path);
    loop {
        sleep(step);
        let utc = Utc::now();

        // Read Files
        let mut days = read_file(days_path).unwrap();
        let mut weeks = read_file(weeks_path).unwrap();
        let mut months = read_file(months_path).unwrap();

        // Get current date, week and month
        let day_current = utc.date().format("%Y-%m-%d").to_string();
        let week_current = format!("{:?}", utc.iso_week());
        let month_current = utc.date().format("%Y-%m").to_string();

        // If this is a new day, week or month, create a new Map entry with value MINUTES_STEP
        // Else, add MINUTES_STEP to days and month
        days = check_and_insert(days, day_current);
        weeks = check_and_insert(weeks, week_current);
        months = check_and_insert(months, month_current);

        // Write days, weeks and months to file
        fs::write(days_path, to_string_pretty(&days).unwrap()).expect("Unable to write to file");
        fs::write(weeks_path, to_string_pretty(&weeks).unwrap()).expect("Unable to write to file");
        fs::write(months_path, to_string_pretty(&months).unwrap())
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

#[allow(dead_code)]
fn json_to_csv(path_json: &str, path_csv: &str) {
    // Converts path_json to csv

    let data = fs::read_to_string(path_json).unwrap();
    let parsed: Value = serde_json::from_str(&data).unwrap();
    let obj: Map<String, Value> = parsed.as_object().unwrap().clone();

    let mut wtr = Writer::from_path(path_csv).unwrap();
    wtr.write_record(&["date", "minutes"]).unwrap();
    for item in obj.iter() {
        wtr.write_record(&[item.0, &item.1.as_i64().unwrap().to_string()])
            .unwrap();
    }
}

#[allow(dead_code)]
fn add_json(first_json_path: &str, second_json_path: &str, result_path: &str) {
    // Creates a new json with the sum of the values of first and second json

    let first_parsed: Value =
        serde_json::from_str(&fs::read_to_string(first_json_path).unwrap()).unwrap();
    let first: Map<String, Value> = first_parsed.as_object().unwrap().clone();

    let second_parsed: Value =
        serde_json::from_str(&fs::read_to_string(second_json_path).unwrap()).unwrap();
    let mut second: Map<String, Value> = second_parsed.as_object().unwrap().clone();

    let mut result: Map<String, Value> = Map::new();

    // Add every item in first to second and put it in result
    for item in first.iter() {
        let key: &String = item.0;
        let val: i64 = item.1.as_i64().unwrap();
        if second.contains_key(key) {
            let other_val = second.get(key).unwrap().as_i64().unwrap();
            result.insert(key.to_string(), json!(val + other_val));
            second.remove(key);
        } else {
            result.insert(key.to_string(), json!(val));
        }
    }

    // Add all remaining items in second to result
    for item in second.iter() {
        let key: &String = item.0;
        let val: i64 = item.1.as_i64().unwrap();
        result.insert(key.to_string(), json!(val));
    }

    fs::write(result_path, to_string_pretty(&result).unwrap()).expect("Unable to write to file");
}

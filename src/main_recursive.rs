use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;

use serde_json::{json, Value};

fn flatten_json(value: &Value, index: &str, flat_map: &mut HashMap<String, Value>, b_sparse: &bool) {
    match value {
        Value::Null => {
            if !b_sparse {
                let key = format!("{}", index);
                flat_map.insert(key, Value::Null);
            }
        }
        Value::Bool(val) => {
            let key = format!("{}", index);
            flat_map.insert(key, json!(*val));
        }
        Value::Number(val) => {
            let key = format!("{}", index);
            flat_map.insert(key, json!(val));
        }
        Value::String(val) => {
            let key = format!("{}", index);
            flat_map.insert(key, json!(val));
        }
        Value::Array(arr) => {
            for (i, v) in arr.iter().enumerate() {
                let idx = format!("{}.{}", index, i);
                flatten_json(v, &idx, flat_map, &b_sparse);
            }
        }
        Value::Object(obj) => {
            for (k, v) in obj.iter() {
                let idx = if index.is_empty() {
                    k.to_owned()
                } else {
                    format!("{}.{}", index, k)
                };
                flatten_json(v, &idx, flat_map, &b_sparse);
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    
    // set the sparse variable: true = no null keys is value is null
    let b_sparse:bool = true;

    // Read the JSON file into a string
    // let mut file = File::open("test_data/lambda_project_testdata_short_with_headerblock.json")?;
    let mut file = File::open("/Users/gcattabriga/downloads/2023-03-01_anthem_index.json")?;
    // let mut file = File::open("test_data/lambda_project_testdata_long.json")?;
    let mut contents = String::new();

    // let tabnine complete the following line 
    file.read_to_string(&mut contents)?;

    // Parse the JSON string into a serde_json::Value
    let json: Value = serde_json::from_str(&contents)?;

    // time the flatten_json function
    let start = Instant::now();


    // Flatten the JSON into a HashMap of flattened keys and associated values
    let mut flat_map = HashMap::new();

    flatten_json(&json, "", &mut flat_map, &b_sparse);

    let duration = start.elapsed();

    // Convert the HashMap to a serde_json::Value and write it to a file
    let json_output = json!(flat_map);
    // let mut output_file = File::create("json_hash_output_short_rust.json")?;
    // let mut output_file = File::create("json_hash_output_long_rust.json")?;
    let mut output_file = File::create("/Users/gcattabriga/downloads/hashed_2023-03-01_anthem_index.json")?;
    output_file.write_all(serde_json::to_string_pretty(&json_output)?.as_bytes())?;

    println!(
        "Flattened JSON in {} microseconds",
        duration.as_micros()
    );
    // println!("duration: {:?}", duration);

    Ok(())
}

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;

use serde_json::Value;

fn flatten_json(value: &Value, index: &str, flat_map: &mut HashMap<String, String>) {
    match value {
        Value::Null => {}
        Value::Bool(_) => {}
        Value::Number(_) => {}
        Value::String(val) => {
            flat_map.insert(index.to_owned(), val.to_owned());
        }
        Value::Array(arr) => {
            for (i, v) in arr.iter().enumerate() {
                let idx = format!("{}[{}]", index, i);
                flatten_json(v, &idx, flat_map);
            }
        }
        Value::Object(obj) => {
            for (k, v) in obj.iter() {
                let idx = if index.is_empty() {
                    k.to_owned()
                } else {
                    format!("{}.{}", index, k)
                };
                flatten_json(v, &idx, flat_map);
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    // Read the JSON file into a string
    let mut file = File::open("test_data/lambda_project_testdata_long.json")?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    // Parse the JSON string into a serde_json::Value
    let json: Value = serde_json::from_str(&contents)?;

    // Flatten the JSON into a hash map of flattened keys and associated values
    let mut flat_map: HashMap<String, String> = HashMap::new();
    flatten_json(&json, "", &mut flat_map);

    // Print the flattened hash map
    for (key, value) in flat_map.iter() {
        println!("{}: {}", key, value);
    }

    Ok(())
}

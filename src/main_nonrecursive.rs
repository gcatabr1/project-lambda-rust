/*  non-recursive flattening function
    processes the json file: test_data/lambda_project_testdata_long.json
    in 8129 microseconds
    processes the json file: test_data/lambda_project_testdata_long_allyears.json
    in 110726 microseconds
*/
use std::collections::HashMap;
use serde_json::Value;
use std::fs::File;
use std::io::Write;
use std::time::Instant;


fn flatten_json(json: Value, b_sparse: &bool) -> HashMap<String, String> {
    let mut stack = vec![("".to_owned(), json)];    
    let mut flattened = HashMap::new();

    while let Some((prefix, json)) = stack.pop() {
        match json {
            Value::Null => {
                if !b_sparse {
                    flattened.insert(prefix, "null".to_owned());
                }
            }
            Value::Bool(b) => {
                flattened.insert(prefix, b.to_string());
            }
            Value::Number(n) => {
                flattened.insert(prefix, n.to_string());
            }
            Value::String(s) => {
                flattened.insert(prefix, s.to_owned());
            }
            Value::Array(arr) => {
                for (i, val) in arr.into_iter().enumerate() {
                    stack.push((format!("{}.{}", prefix, i), val));
                }
            }
            Value::Object(obj) => {
                for (key, val) in obj.into_iter() {
                    if !prefix.is_empty() {
                        stack.push((format!("{}.{}", prefix, key), val));
                    } else {
                        stack.push((key, val));
                    }  
                }
            }           
        }
    }

    flattened
}

fn main() -> std::io::Result<()> {
    let input_file = "test_data/lambda_project_testdata_long_allyears.json";
    let output_file = "json_hash_output_long_all_years_rust.json";

    // set the sparse variable: true = no null keys is value is null
    let b_sparse:bool = true;

    // Read input file
    let input_json: Value = serde_json::from_reader(File::open(input_file)?)?;

    // time the flatten_json function
    let start = Instant::now();

    // Flatten the JSON object into a HashMap
    let flattened = flatten_json(input_json, &b_sparse);

    let duration = start.elapsed();

    // Write flattened data to output file
    let mut output_file = File::create(output_file)?;
    
    let output_json = serde_json::to_string(&flattened)?;
    output_file.write_all(output_json.as_bytes())?;

    println!(
        "Flattened JSON in {} microseconds",
        duration.as_micros()
    );

    Ok(())
}

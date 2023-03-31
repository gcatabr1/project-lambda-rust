// This was hard! 
// Particullarly the non-recursive vec function. Had to experiment with a lot of different concepts, 
// of which I still don't fully understand
// Had to get the derive macro correct in the cargo.toml correct: serde = { version = "1.0.155", features = ["derive"] }


mod flatteners;

use std::collections::HashMap;
use std::fs::File;
use std::io::{Write, BufWriter};
// use std::io::prelude::*;
use std::time::Instant;
use serde_json::{json, Value};

use crate::flatteners::recurs::recurs::*;
use crate::flatteners::nonrecurs_vec::nonrecurs_vec::*;
use crate::flatteners::nonrecurs::nonrecurs::*;

fn main() -> std::io::Result<()> {

    let input_file = "test_data/lambda_project_testdata_long_allyears.json";
    let output_file_recurs = "json_hash_output_allyears_recurs_rust.json";  
    let output_file_nonrecurs = "json_hash_output_allyears_nonrecurs_rust.json";   
    let output_file_nonrecurs_vec = "json_hash_output_allyears_nonrecurs_vec_rust.json";        
    
    // set the sparse variable: true = no null keys is value is null
    let b_sparse:bool = true;


    // Read input file into json structure
    let json: Value = serde_json::from_reader(File::open(input_file)?)?;

    // Flatten the JSON into a HashMap of flattened keys and associated values
    // Create a flat_map to put the flat hash map into
    let mut flat_map = HashMap::new();

    // -----------------------------------------------------------
    // RECURSIVE flattening
    // -----------------------------------------------------------
    // time the flatten_json_recurs function
    let start = Instant::now();

    flatten_json_recurs(&json, "", &mut flat_map, &b_sparse);

    let duration = start.elapsed();

    // Convert the HashMap to a serde_json::Value and write it to a file
    let json_output = json!(flat_map);

    // let mut output_file = File::create("json_hash_output_short_rust.json")?;
    let mut output_file_recurs = File::create(output_file_recurs)?;

    // let's pretty print the json output
    output_file_recurs.write_all(serde_json::to_string_pretty(&json_output)?.as_bytes())?;

    println!(
        "Flattened JSON recursively in {} microseconds",
        duration.as_micros()
    );

    // -----------------------------------------------------------
    // NON-RECURSIVE flattening
    // -----------------------------------------------------------
    // let's clone the json structure, and not do it in flatten function 
    let json_clone = json.clone();

    // time the flatten_json_nonrecurs function
    let start = Instant::now();

    // Flatten the JSON object into a HashMap
    let flattened = flatten_json_nonrecurs(json_clone, &b_sparse);

    let duration = start.elapsed();

    // Write flattened data to output file
    let mut output_file_nonrecurs = File::create(output_file_nonrecurs)?;
    let json_output = json!(flattened);

    // let's pretty print the json output
    output_file_nonrecurs.write_all(serde_json::to_string_pretty(&json_output)?.as_bytes())?;

    println!(
        "Flattened JSON non-recursively in {} microseconds",
        duration.as_micros()
    );


    // -----------------------------------------------------------
    // NON-RECURSIVE flattening using a Vec instead of HashMap
    // -----------------------------------------------------------
    // time the flatten_json_nonrecurs function
    let start = Instant::now();

    // Flatten the JSON object into a Vec 
    // not cloning saves a ton of time. that is, json v json.clone()
    let flattened = flatten_json_nonrecurs_vec(json, &b_sparse);

    let duration = start.elapsed();

    // create the file pointer for the output file
    let mut output_file_nonrecurs_vec = File::create(output_file_nonrecurs_vec)?;

    // convert the returned vec to a json representation
    // let flattened_json_value = serde_json::to_value(&flattened).unwrap();

    let mut buf_writer = BufWriter::new(&mut output_file_nonrecurs_vec);

    buf_writer.write_all(b"{\n")?;

    for (index, (key, value)) in flattened.iter().enumerate() {
        let line = format!("  \"{}\": {}", key, value);
        buf_writer.write_all(line.as_bytes()).expect("Unable to write data to file");
        if index < flattened.len() - 1 {
            buf_writer.write_all(b",\n").expect("Unable to write data to file");
        } else {
            buf_writer.write_all(b"\n").expect("Unable to write data to file");
        }
    }

    buf_writer.write_all(b"}").expect("Unable to write data to file");

    println!(
        "Flattened JSON non-recursively using Vec in {} microseconds",
        duration.as_micros()
    );    


    Ok(())
}
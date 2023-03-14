use std::collections::HashMap;
use serde_json::{json, Value};


// recursive flattener
pub fn flatten_json_recurs(value: &Value, index: &str, flat_map: &mut HashMap<String, Value>, b_sparse: &bool) {
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
                flatten_json_recurs(v, &idx, flat_map, &b_sparse);
            }
        }
        Value::Object(obj) => {
            for (k, v) in obj.iter() {
                let idx = if index.is_empty() {
                    k.to_owned()
                } else {
                    format!("{}.{}", index, k)
                };
                flatten_json_recurs(v, &idx, flat_map, &b_sparse);
            }
        }
    }
}


// non recursive flattener
pub fn flatten_json_nonrecurs(json: Value, b_sparse: &bool) -> HashMap<String, Value> {
    let mut stack = vec![("".to_owned(), json)];    
    let mut flattened: HashMap<String, Value> = HashMap::new();

    while let Some((prefix, json)) = stack.pop() {
        match json {
            Value::Null => {
                if !b_sparse {
                    flattened.insert(prefix, Value::Null);
                }
            }
            Value::Bool(b) => {
                flattened.insert(prefix, json!(b));
            }
            Value::Number(n) => {
                flattened.insert(prefix, json!(n));
            }
            Value::String(s) => {
                flattened.insert(prefix, json!(s));
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
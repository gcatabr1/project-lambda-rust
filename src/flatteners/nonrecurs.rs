// create module for nonrecursive functions
pub mod nonrecurs {

    use std::collections::HashMap;
    use serde_json::{json, Value};


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


}
//create a module for recursive functions
pub mod recurs {

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
}


#[cfg(test)]
mod tests {

    use crate::flatteners::recurs::recurs::*;
    use std::collections::HashMap;
    use serde_json::{json, Value};

    #[test]
    fn test_json_flatten_json_recurs() {

        let test_json = r#"
        {
            "contents":[
               {
                  "member_id":"member78103",
                  "member_age":8,
                  "member_sex":"F",
                  "claim":[
                     {
                        "claim_id":"claim_22250000",
                        "claim_type":"P",
                        "diagnosis_codes":[
                           "Q6231"
                        ],
                        "claim_line":[
                           {
                              "line_number":1,
                              "from_date":"2020-04-01",
                              "procedure_code":"99212",
                              "quantity":1,
                              "allowed_amount":79.28
                           }
                        ]
                     }
                  ]   
            }
          ]
        }
        "#;

        let test_expected = r#"
            {"contents.0.claim.0.claim_id":"claim_22250000","contents.0.claim.0.claim_line.0.allowed_amount":79.28,"contents.0.claim.0.claim_line.0.from_date":"2020-04-01","contents.0.claim.0.claim_line.0.line_number":1,"contents.0.claim.0.claim_line.0.procedure_code":"99212","contents.0.claim.0.claim_line.0.quantity":1,"contents.0.claim.0.claim_type":"P","contents.0.claim.0.diagnosis_codes.0":"Q6231","contents.0.member_age":8,"contents.0.member_id":"member78103","contents.0.member_sex":"F"}
        "#;       

        let b_sparse:bool = true;

        let v: Value = serde_json::from_str(test_json).unwrap();

        let mut flat_map: HashMap<String, Value> = HashMap::new();

        flatten_json_recurs(&v, "", &mut flat_map, &b_sparse);

        let json_output = json!(flat_map);
        let json_expected: Value = serde_json::from_str(test_expected).unwrap();

        assert_eq!(json_output, json_expected);
    }
}


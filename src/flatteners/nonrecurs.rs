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


#[cfg(test)]
mod tests {

    use crate::flatteners::nonrecurs::nonrecurs::*;
    use serde_json::{json, Value};

    #[test]
    fn test_json_flatten_json_nonrecurs() {

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

        let test_value: Value = serde_json::from_str(test_json).unwrap();

        let b_sparse:bool = true;

        let flattened = flatten_json_nonrecurs(test_value, &b_sparse);

        let json_output = json!(flattened);

        let json_expected: Value = serde_json::from_str(test_expected).unwrap();

        assert_eq!(json_output, json_expected);
    }
}

// create module for nonrecursive vec functions
pub mod nonrecurs_vec {

    use serde_json::{Value};
    use serde::{Deserialize, Serialize, Serializer};

    #[derive(Debug, Deserialize)]
    pub enum FlatValue {
        Null,
        Bool(bool),
        String(String),
        Number(f64),
    }

    // non recursive flattener using vec 
    pub fn flatten_json_nonrecurs_vec(json: Value, b_sparse: &bool) -> Vec<(String, FlatValue)> {
        let mut flattened = Vec::new();
        let mut stack = vec![(json, "".to_string())];

        while !stack.is_empty() {
            let (value, prefix) = stack.pop().unwrap();
            match value {
                Value::Object(map) => {
                    for (key, value) in map {
                        let new_key = if prefix.is_empty() {
                            key.clone()
                        } else {
                            format!("{}.{}", prefix, key)
                        };
                        stack.push((value, new_key));
                    }
                }
                Value::Array(array) => {
                    for (index, value) in array.into_iter().enumerate() {
                        let new_key = format!("{}.{}", prefix, index);
                        stack.push((value, new_key));
                    }
                }
                Value::Null => {
                    if !b_sparse {
                        flattened.push((prefix, FlatValue::Null));
                    }
                }
                Value::Bool(b) => {
                    flattened.push((prefix, FlatValue::Bool(b)));

                }
                Value::String(s) => {
                    flattened.push((prefix, FlatValue::String(s)));
                }
                Value::Number(num) => {
                    if let Some(n) = num.as_f64() {
                        flattened.push((prefix, FlatValue::Number(n)));
                    }
                }
            }
        }

        flattened
    }


    impl Serialize for FlatValue {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            match self {
                FlatValue::Null => serializer.serialize_unit(),
                FlatValue::Bool(b) => serializer.serialize_bool(*b),
                FlatValue::String(s) => serializer.serialize_str(s),
                FlatValue::Number(n) => serializer.serialize_f64(*n),
            }
        }
    }

    impl std::fmt::Display for FlatValue {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                FlatValue::Null => write!(f, "null"),
                FlatValue::Bool(b) => write!(f, "{}", b),
                FlatValue::String(s) => write!(f, "\"{}\"", s),
                FlatValue::Number(n) => write!(f, "{}", n),
            }
        }
    }
}


#[cfg(test)]
mod tests {

    use crate::flatteners::nonrecurs_vec::nonrecurs_vec::*;
    use serde_json::{json, Value};

    #[test]
    fn test_json_flatten_json_nonrecurs_vec() {

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
        [["contents.0.member_sex","F"],["contents.0.member_id","member78103"],["contents.0.member_age",8.0],["contents.0.claim.0.diagnosis_codes.0","Q6231"],["contents.0.claim.0.claim_type","P"],["contents.0.claim.0.claim_line.0.quantity",1.0],["contents.0.claim.0.claim_line.0.procedure_code","99212"],["contents.0.claim.0.claim_line.0.line_number",1.0],["contents.0.claim.0.claim_line.0.from_date","2020-04-01"],["contents.0.claim.0.claim_line.0.allowed_amount",79.28],["contents.0.claim.0.claim_id","claim_22250000"]]
        "#;       

        let test_value: Value = serde_json::from_str(test_json).unwrap();
        let b_sparse:bool = true;

        let flattened = flatten_json_nonrecurs_vec(test_value, &b_sparse);

        let json_output = json!(flattened);

        let json_expected: Value = serde_json::from_str(test_expected).unwrap();

        assert_eq!(json_output, json_expected);
    }
}

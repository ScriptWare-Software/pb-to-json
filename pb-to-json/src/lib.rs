/// Converts a protocol buffer string to a JSON string.
///
/// The function takes a protocol buffer string as input and returns the corresponding JSON string.
/// It supports nested objects, repeated fields, and key-value pairs of different types (string, number, boolean).
///
/// # Examples
///
/// ```
/// use pb_to_json::convert_pb_to_json;
/// 
/// let pb_string = r#"
///     name: "John Doe"
///     age: 30
///     city: "New York"
/// "#;
///
/// let json_string = convert_pb_to_json(pb_string);
/// println!("{}", json_string);
/// ```
///
/// The above example will output:
/// ```json
/// {
///     "name": "John Doe",
///     "age": "30",
///     "city": "New York"
/// }
/// ```
pub fn convert_pb_to_json(pb_string: &str) -> String {
    let mut data = serde_json::Map::new();
    let mut current_object: Option<serde_json::Map<String, serde_json::Value>> = None;
    let mut current_key = String::new();

    for line in pb_string.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // check if the line ends with "{", indicating the start of a nested object
        if line.ends_with("{") {
            current_key = line[..line.len() - 2].trim().to_string();
            current_object = Some(serde_json::Map::new());
        } else if line == "}" {
            if let Some(obj) = current_object.take() {
                data.insert(current_key.clone(), serde_json::Value::Object(obj));
            }
        } else {
            let parts: Vec<&str> = line.split(':').map(|s| s.trim()).collect();
            
            // skip the line if it doesn't have exactly two parts (key and value)
            if parts.len() != 2 {
                continue;
            }

            let key = parts[0].trim();
            let value = parts[1].trim().trim_matches('"');

            if let Some(ref mut obj) = current_object {
                obj.insert(key.to_string(), serde_json::Value::String(value.to_string()));
            } else if data.contains_key(key) {
                if let Some(serde_json::Value::Array(ref mut arr)) = data.get_mut(key) {
                    arr.push(serde_json::Value::String(value.to_string()));
                } else {
                    let mut arr = Vec::new();
                    arr.push(data[key].clone());
                    arr.push(serde_json::Value::String(value.to_string()));
                    data.insert(key.to_string(), serde_json::Value::Array(arr));
                }
            } else {
                data.insert(key.to_string(), serde_json::Value::String(value.to_string()));
            }
        }
    }

    serde_json::to_string_pretty(&data).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_pb() {
        let pb_string = r#"
            name: "John Doe"
            age: 30
            city: "New York"
        "#;

        let expected_json = r#"{
            "name": "John Doe",
            "age": "30",
            "city": "New York"
        }"#;

        let actual_json = convert_pb_to_json(pb_string);
        assert_eq!(normalize_json(expected_json), normalize_json(&actual_json));
    }

    #[test]
    fn test_nested_pb() {
        let pb_string = r#"
            name: "Alice"
            address {
                street: "123 Main St"
                city: "London"
                country: "UK"
            }
            phone_numbers: "1234567890"
            phone_numbers: "9876543210"
        "#;

        let expected_json = r#"{
            "name": "Alice",
            "address": {
                "street": "123 Main St",
                "city": "London",
                "country": "UK"
            },
            "phone_numbers": [
                "1234567890",
                "9876543210"
            ]
        }"#;

        let actual_json = convert_pb_to_json(pb_string);
        assert_eq!(normalize_json(expected_json), normalize_json(&actual_json));
    }

    #[test]
    fn test_empty_pb() {
        let pb_string = "";
        let expected_json = "{}";

        let actual_json = convert_pb_to_json(pb_string);
        assert_eq!(expected_json, actual_json);
    }

    fn normalize_json(json: &str) -> String {
        let parsed: serde_json::Value = serde_json::from_str(json).unwrap();
        serde_json::to_string_pretty(&parsed).unwrap()
    }
}
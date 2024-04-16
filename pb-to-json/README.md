# pb-to-json

`pb_to_json` is a Rust library for converting protocol buffer strings to JSON. It provides a simple and efficient way to transform protocol buffer data into a JSON representation. That's it.

Features
* Converts protocol buffer strings to JSON
* Supports nested objects and repeated fields
* Handles key-value pairs of different types (string, number, boolean)
* Provides a user-friendly API

## Installation
To use this basic library, add the following to your `Cargo.toml` file:

```toml
[dependencies]
pb_to_json = "0.1.1"
```

## Documentation
Documentation is available at docs.rs, you can find it here.

## Usage
Here's a basic example of how to use `pb_to_json`:

```rust
use pb_to_json::convert_pb_to_json;

fn main() {
    let pb_string = r#"
        name: "John Doe"
        age: 30
        city: "New York"
    "#;

    let json_string = convert_pb_to_json(pb_string);
    println!("{}", json_string);
}
```

### Output
```json
{
    "name": "John Doe",
    "age": "30",
    "city": "New York"
}
```

## Contributions
Found a way to improve this library? Open an issue or submit a pull request to the repository! We're happy to review it.

## License
`pb_to_json` is licensed under the MIT license. 
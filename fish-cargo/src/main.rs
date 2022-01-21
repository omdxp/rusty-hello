extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

// use serde_json::Value as JsonValue;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

fn main() {
    let json = r#"
        {
            "name": "Omar",
            "age": 23,
            "phones": [
                "+48 1234567",
                "+48 2345678"
            ]
        }
    "#;

    let data = serde_json::from_str(json);

    if data.is_ok() {
        let data: Person = data.unwrap();

        println!("{}", data.name);
        println!("{}", data.age);
        println!("{:?}", data.phones);
    } else {
        println!("Error: {:?}", data.err());
    }
}

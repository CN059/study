
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
struct Person {
    name: String,
    age: u8,
}

#[test]
fn test_serialize() {
    let person = Person {
        name: "Alice".to_string(),
        age: 30,
    };
    let json = serde_json::to_string(&person).unwrap();
    println!("{}", json);
    assert_eq!(json, r#"{"name":"Alice","age":30}"#);
}

#[test]
fn test_deserialize() {
    let json = r#"{"name":"Bob","age":25}"#;
    let person: Person = serde_json::from_str(json).unwrap();
    assert_eq!(
        person,
        Person {
            name: "Bob".to_string(),
            age: 25
        }
    );
}

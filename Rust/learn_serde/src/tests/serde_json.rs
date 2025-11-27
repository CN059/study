
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

// serde其实就是一个通用的序列化框架，可以用于多种格式
// 如果只使用serde,是不能进行我们想要的序列化的
// 使用其他的依赖于serde的库进行序列化
#[test]
fn test_serialize_toml(){
    let person = Person {
        name: "Charlie".to_string(),
        age: 28,
    };
    let toml = toml::to_string(&person).unwrap();
    println!("{}", toml);
    assert_eq!(toml, r#"name = "Charlie"
age = 28
"#);
}

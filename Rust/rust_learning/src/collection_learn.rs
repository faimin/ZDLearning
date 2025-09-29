use std::collections::HashMap;

pub fn map_test() {
    let mut map = HashMap::new();
    map.insert("key", String::from("value"));

    let v = match map.get("key") {
        Some(str_value) => str_value,
        None => "not found"
    };
    println!("{}", v);
}
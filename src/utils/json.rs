use serde::de::DeserializeOwned;
use serde_json::Value;

pub fn convert_value_vec_to_string_vec(v: Vec<Value>) -> Vec<String> {
    v.into_iter()
        .map(|i| match i {
            Value::String(s) => Some(s),
            _ => None,
        })
        .filter(|i| match i {
            Some(_) => true,
            None => false,
        })
        .map(|i| i.unwrap())
        .collect()
}

pub fn deserialize_vec<T: DeserializeOwned>(v: Vec<Value>) -> Vec<T> {
    v.into_iter()
        .map(|i| serde_json::from_value(i).unwrap())
        .collect()
}

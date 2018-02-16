#[macro_use]
extern crate serde_json;

use std::string::String;
use serde_json::{Map, Value};
use std::vec::Vec;

/// Iterate recursively through the map and build a flat dot map from the leafs
fn traverse_dot(src: &Map<String, Value>, dest: &mut Map<String, Value>, inc_path: &Vec<String>) {
    for (key, value) in src {
        // Create a new path including this iteration's key
        let mut path = inc_path.clone();

        path.push(key.clone());

        match value {
            &Value::Array(ref a) => {
                // Create new paths including the array indices
                let mut m = Map::new();

                let mut cnt = 0;
                for v in a {
                    m.insert(cnt.to_string(), v.clone());

                    cnt += 1;
                }

                traverse_dot(&m, dest, &path);
            },
            &Value::Object(ref m) => {
                traverse_dot(m, dest, &path);
            },
            &Value::Null => {
                dest.insert(path.join("."), Value::Null);
            },
            &Value::String(ref s) => {
                dest.insert(path.join("."), Value::String(s.clone()));
            },
            &Value::Number(ref n) => {
                dest.insert(path.join("."), Value::Number(n.clone()));
            },
            &Value::Bool(ref b) => {
                dest.insert(path.join("."), Value::Bool(*b));
            },
        }
    }
}

/// Create a flat dot map from a deep serde_json::Map
///
/// # Examples
///
/// ```rust
/// # #[macro_use]
/// # extern crate serde_json;
/// # extern crate dot_json;
/// use serde_json::{Map, Value, from_str};
/// # use dot_json::to_dot;
/// #
/// # fn main() {
///
/// let data = r#"{
///                 "foo": "Lorem ipsum",
///                 "bar": [null, 123, true],
///                 "baz": { "qux": 789 }
///               }"#;
/// let value: Value = from_str(data).unwrap();
///
/// if let Value::Object(map) = value {
///     let dot_map = to_dot(&map);
///
///     assert_eq!(5, dot_map.len());
///     assert_eq!(Some(&Value::String("Lorem ipsum".to_string())), dot_map.get("foo"));
///     assert_eq!(Some(&Value::Null), dot_map.get("bar.0"));
///     assert_eq!(Some(&Value::Number(123.into())), dot_map.get("bar.1"));
///     assert_eq!(Some(&Value::Bool(true)), dot_map.get("bar.2"));
///     assert_eq!(Some(&Value::Number(789.into())), dot_map.get("baz.qux"));
/// }
/// # }
/// ```
pub fn to_dot(src: &Map<String, Value>) -> Map<String, Value> {
    let mut dest = Map::new();

    traverse_dot(&src, &mut dest, &vec![]);

    dest
}
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
                traverse_dot(&arr_to_dot_map(&a), dest, &path);
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

/// Create a flat dot map from a deep `serde_json::Map`
///
/// # Example
///
/// ```rust
/// # #[macro_use]
/// # extern crate serde_json;
/// # extern crate dot_json;
/// use serde_json::{Map, Value, from_str};
/// # use dot_json::map_to_dot_map;
/// #
/// # fn main() {
///
/// let data = json!({
///     "foo": "Lorem ipsum",
///     "bar": [null, 123, true],
///     "baz": { "qux": 789 }
/// });
///
/// if let Value::Object(map) = data {
///     let dot_map = map_to_dot_map(&map);
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
pub fn map_to_dot_map(src: &Map<String, Value>) -> Map<String, Value> {
    let mut dest = Map::new();

    traverse_dot(&src, &mut dest, &vec![]);

    dest
}

/// Create a flat dot map from a deep `Vec<serde_json::Value>`
///
/// # Example
///
/// ```rust
/// # #[macro_use]
/// # extern crate serde_json;
/// # extern crate dot_json;
/// use serde_json::{Map, Value, from_str};
/// # use dot_json::arr_to_dot_map;
/// #
/// # fn main() {
///
/// let data = json!([
///     "Lorem ipsum",
///     [null, 123, true],
///     { "qux": 789 }
/// ]);
///
/// if let Value::Array(arr) = data {
///     let dot_map = arr_to_dot_map(&arr);
///
///     assert_eq!(Value::String("Lorem ipsum".to_string()), dot_map["0"]);
///     assert_eq!(Value::Bool(true), dot_map["1.2"]);
/// }
/// # }
/// ```
pub fn arr_to_dot_map(src: &Vec<Value>) -> Map<String, Value> {
    let mut m = Map::new();

    let mut cnt = 0;
    for v in src {
        m.insert(cnt.to_string(), v.clone());

        cnt += 1;
    }

    map_to_dot_map(&m)
}

/// Create a flat dot map from a serde_json::Value::{Array, Object} while just cloning any other value
///
/// # Examples
///
/// ```rust
/// # #[macro_use]
/// # extern crate serde_json;
/// # extern crate dot_json;
/// use serde_json::{Map, Value, from_str};
/// # use dot_json::value_to_dot;
/// #
/// # fn main() {
///
/// let data = json!({
///     "foo": "Lorem ipsum",
///     "bar": [null, 123, true],
///     "baz": { "qux": 789 }
/// });
///
/// let dot_map = value_to_dot(&data);
///
/// assert_eq!(Value::String("Lorem ipsum".to_string()), dot_map["foo"]);
/// assert_eq!(Value::Null, dot_map["bar.0"]);
/// assert_eq!(Value::Number(123.into()), dot_map["bar.1"]);
/// assert_eq!(Value::Bool(true), dot_map["bar.2"]);
/// assert_eq!(Value::Number(789.into()), dot_map["baz.qux"]);
/// # }
/// ```
///
/// ```rust
/// # extern crate serde_json;
/// # extern crate dot_json;
/// use serde_json::{Map, Value, from_str};
/// # use dot_json::value_to_dot;
/// #
/// # fn main() {
/// assert_eq!(Value::String("Lorem ipsum".to_string()), value_to_dot(&Value::String("Lorem ipsum".to_string())));
/// assert_eq!(Value::Null, value_to_dot(&Value::Null));
/// assert_eq!(Value::Number(123.into()), value_to_dot(&Value::Number(123.into())));
/// assert_eq!(Value::Bool(true), value_to_dot(&Value::Bool(true)));
///
/// let dot_map = value_to_dot(&Value::Array(vec![
///     Value::Bool(true),
///     Value::Bool(false),
///     Value::Array(vec![
///         Value::Number(123.into()),
///     ]),
/// ]));
///
/// assert_eq!(Value::Bool(true), dot_map["0"]);
/// assert_eq!(Value::Bool(false), dot_map["1"]);
/// assert_eq!(Value::Number(123.into()), dot_map["2.0"]);
/// # }
/// ```
pub fn value_to_dot(src: &Value) -> Value {
    match src {
        &Value::Array(ref a) => Value::Object(arr_to_dot_map(a)),
        &Value::Object(ref m) => Value::Object(map_to_dot_map(m)),
        &Value::Null => Value::Null,
        &Value::String(ref s) => Value::String(s.clone()),
        &Value::Number(ref n) => Value::Number(n.clone()),
        &Value::Bool(ref b) => Value::Bool(*b),
    }
}


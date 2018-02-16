# dot_json

Utilities for working with `serde_json::Map` structures as "dot maps".

```
[dependencies]
dot_json = "0.1"
```

## map_to_dot

Convert a deep `serde_json::Map` into a shallow dot map, from:

```json
{
    "foo": "Lorem ipsum",
    "bar": [123, null, { "baz": "Dolor sit amet" }],
    "qux": {
        "deep": {
            "one": true,
            "two": false,
        }
    }
}
```

to:

```json
{
    "foo": "Lorem ipsum",
    "bar.0": 123,
    "bar.1": null,
    "bar.2.baz": "Dolor sit amet",
    "qux.deep.one": true,
    "qux.deep.two": false
}
```

```rust
extern crate dot_json;
extern crate serde_json;

use dot_json::map_to_dot;
use serde_json::{Map, Value, Error};

fn example() -> Result<(), Error> {
    let data = r#"{
                      "foo": "Lorem ipsum",
                      "bar": [123, null, { "baz": "Dolor sit amet" }],
                      "qux": {
                          "deep": {
                              "one": true,
                              "two": false,
                          }
                      }
                  }"#;

    let value: Value = serde_json::from_str(data)?;

    if let Value::Object(map) = value {
        let dot_map = map_to_dot(&map);

        assert_eq!(
            Some(&Value::Bool(false)),
            dot_map.get("qux.deep.two")
        );
    }

    Ok(())
}
```

## value_to_dot

Convert a `serde_json::Value` into a shallow dot map, if possible, otherwise clone:

* `Value::Array -> Value::Object` Array gets converted into a map where keys are indices, and is then run through `map_to_dot`
* `Value::Object -> Value::Object` Is run through `map_to_dot`
* Other values: No change - gets cloned
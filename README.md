# dot_json

Utilities for working with `serde_json::Map` structures as "dot maps".

```
[dependencies]
dot_json = "0.1"
```

## to_dot

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

use dot_json::to_dot;
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
        let dot_map = to_dot(&map);

        assert_eq!(
            Some(&Value::Bool(false)),
            dot_map.get("qux.deep.two")
        );
    }

    Ok(())
}
```
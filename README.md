# dot\_json

Utilities for working with `serde_json::Map` structures as "dot maps".

```
[dependencies]
dot_json = "0.2"
```

## value\_to\_dot

See also: [https://github.com/serde-rs/json#constructing-json-values](https://github.com/serde-rs/json#constructing-json-values)

```rust
#[macro_use]
extern crate serde_json;
extern crate dot_json;

use dot_json::value_to_dot;
use serde_json::{Map, Value};

fn main() {
	let obj = json!({
		"foo": "Lorem ipsum",
		"bar": [123, null, { "baz": "Dolor sit amet" }],
		"qux": {
			"deep": {
				"one": true,
				"two": false,
			}
		}
	});
		
	let obj_dot = value_to_dot(&obj);
	
	// obj_dot is now: {
	//     "foo": "Lorem ipsum",
	//     "bar.0": 123,
	//     "bar.1": null,
	//     "bar.2.baz": "Dolor sit amet",
	//     "qux.deep.one": true,
	//     "qux.deep.two": false
	// }
		
	assert_eq!(Value::Null, obj_dot["bar.1"]);
	assert_eq!(Value::Bool(false), obj_dot["qux.deep.two"]);
		
	let arr = json!([
		{ "foo", false },
		null
	]);
		
	let arr_dot = value_to_dot(&arr);
		
	assert_eq!(Value::Bool(false), arr["0.foo"]);
	assert_eq!(Value::Null, arr["1"]);
}
```

## map\_to\_dot\_map

Used by `value_to_dot` and `arr_to_dot_map` to convert deep `serde_json::Map` to shallow dot maps.

## arr\_to\_dot\_map

Used by `value_to_dot` to convert deep `serde_json::Value::Array` vectors to shallow dot maps.
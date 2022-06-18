# js_option

This crate provides a type `JsOption` that is very similar to the standard
library's `Option` type except that it has three variants:

* `Some(value)`: Like `Option::Some`
* `Null`: Explicitly not some value
* `Undefined`: Implicitly not some value

This type can be useful when you want to deserialize JSON to a Rust struct
and not loose information: A regular `Option` deserializes to `None` from
both an explicit `null` or a missing field (this is due to special casing of
`Option` in the `Deserialize` and `Serialize` derive macros, for other types
a missing field will make deserialization fail unless there is a
`#[serde(skip)]`, `#[serde(skip_deserializing)]` or `#[serde(default)]`
attribute).

## Example:

```
# extern crate serde_crate as serde;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct MyStruct {
    #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
    my_field: JsOption<String>,
}
```

## License

[MIT](https://opensource.org/licenses/MIT)

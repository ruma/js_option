// Undo rename from Cargo.toml
extern crate serde_crate as serde;

use serde::{
    de::{Deserialize, Deserializer},
    ser::{Error as _, Serialize, Serializer},
};

use crate::JsOption;

impl<'de, T> Deserialize<'de> for JsOption<T>
where
    T: Deserialize<'de>,
{
    /// Deserialize a `JsOption`.
    ///
    /// This implementation will never return `Undefined`. You need to use
    /// `#[serde(default)]` to get `Undefined` when the field is not present.
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Option::<T>::deserialize(deserializer).map(Self::from_option)
    }
}

impl<T> Serialize for JsOption<T>
where
    T: Serialize,
{
    /// Serialize a `JsOption`.
    ///
    /// Serialization will fail for `JsOption::Undefined`. You need to use
    /// `#[skip_serializing_if = "JsOption::is_undefined"]` to stop the field
    /// from being serialized altogether.
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            Self::Some(val) => serializer.serialize_some(val),
            Self::Null => serializer.serialize_none(),
            Self::Undefined => Err(S::Error::custom("attempted to serialize `undefined`")),
        }
    }
}

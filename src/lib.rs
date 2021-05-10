//! This crate provides a type `JsOption` that is very similar to the standard
//! library's `Option` type except that it has three variants:
//!
//! * `Some(value)`: Like `Option::Some`
//! * `Null`: Explicitly not some value
//! * `Undefined`: Implicitly not some value
//!
//! This type can be useful when you want to deserialize JSON to a Rust struct
//! and not loose information: A regular `Option` deserializes to `None` from
//! both an explicit `null` or a missing field (this is due to special casing of
//! `Option` in the `Deserialize` and `Serialize` derive macros, for other types
//! a missing field will make deserialization fail unless there is a
//! `#[serde(skip)]`, `#[serde(skip_deserializing)]` or `#[serde(default)]`
//! attribute).
//!
//! # Example:
//!
//! ```
//! # extern crate serde_crate as serde;
//! use js_option::JsOption;
//! use serde::{Deserialize, Serialize};
//!
//! #[derive(Serialize, Deserialize)]
//! # #[serde(crate = "serde")]
//! struct MyStruct {
//!     #[serde(default, skip_serializing_if = "JsOption::is_undefined")]
//!     my_field: JsOption<String>,
//! }
//! ```

#![warn(missing_docs)]

use std::ops::{Deref, DerefMut};

#[cfg(feature = "serde")]
mod serde;

/// An `Option`-like type with two data-less variants in addition to `Some`:
/// `Null` and `Undefined`.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum JsOption<T> {
    /// Some value `T`
    Some(T),
    /// Explicit absence of a value
    Null,
    /// Implicit absence of a value
    Undefined,
}

impl<T> JsOption<T> {
    /// Construct a `JsOption` from a regular `Option`.
    ///
    /// `None` will be converted to to `Null`.
    pub fn from_option(opt: Option<T>) -> Self {
        match opt {
            Some(val) => Self::Some(val),
            None => Self::Null,
        }
    }

    /// Construct a `JsOption` from a regular `Option`.
    ///
    /// `None` will be converted to `Undefined`.
    pub fn from_implicit_option(opt: Option<T>) -> Self {
        match opt {
            Some(val) => Self::Some(val),
            None => Self::Undefined,
        }
    }

    /// Convert a `JsOption` to `Option`.
    pub fn into_option(self) -> Option<T> {
        match self {
            Self::Some(val) => Some(val),
            _ => None,
        }
    }

    /// Convert a `JsOption<T>` to `Option<Option<T>>`.
    ///
    /// `Null` is represented as `Some(None)` while `Undefined` is represented
    /// as `None`.
    pub fn into_nested_option(self) -> Option<Option<T>> {
        match self {
            Self::Some(val) => Some(Some(val)),
            Self::Null => Some(None),
            Self::Undefined => None,
        }
    }

    /// Returns `true` if the `JsOption` contains a value.
    pub const fn is_some(&self) -> bool {
        matches!(self, Self::Some(_))
    }

    /// Returns `true` if the `JsOption` is `Null`.
    pub const fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }

    /// Returns `true` if the `JsOption` is `Undefined`.
    pub const fn is_undefined(&self) -> bool {
        matches!(self, Self::Undefined)
    }

    /// Returns the contained `Some` value, consuming the `self` value.
    ///
    /// # Panics
    ///
    /// Panics if the self value equals `Null` or `Undefined`.
    #[track_caller]
    pub fn unwrap(self) -> T {
        match self {
            Self::Some(val) => val,
            Self::Null => panic!("called `JsOption::unwrap()` on `Null`"),
            Self::Undefined => panic!("called `JsOption::unwrap()` on `Undefined`"),
        }
    }

    /// Returns the contained `Some` value or a provided default.
    pub fn unwrap_or(self, default: T) -> T {
        match self {
            Self::Some(val) => val,
            _ => default,
        }
    }

    /// Returns the contained `Some` value computes is from a closure.
    pub fn unwrap_or_else<F: FnOnce() -> T>(self, f: F) -> T {
        match self {
            Self::Some(val) => val,
            _ => f(),
        }
    }

    /// Maps a `JsOption<T>` to `JsOption<U>` by applying a function to a
    /// contained value.
    pub fn map<U, F: FnOnce(T) -> U>(self, f: F) -> JsOption<U> {
        match self {
            Self::Some(val) => JsOption::Some(f(val)),
            Self::Null => JsOption::Null,
            Self::Undefined => JsOption::Undefined,
        }
    }

    /// Converts from `&Option<T>` to `Option<&T>`.
    pub const fn as_ref(&self) -> JsOption<&T> {
        match self {
            Self::Some(x) => JsOption::Some(x),
            Self::Null => JsOption::Null,
            Self::Undefined => JsOption::Undefined,
        }
    }

    /// Converts from `&mut Option<T>` to `Option<&mut T>`.
    pub fn as_mut(&mut self) -> JsOption<&mut T> {
        match self {
            Self::Some(x) => JsOption::Some(x),
            Self::Null => JsOption::Null,
            Self::Undefined => JsOption::Undefined,
        }
    }
}

impl<T: Default> JsOption<T> {
    /// Returns the contained `Some` value or a default.
    pub fn unwrap_or_default(self) -> T {
        self.unwrap_or_else(Default::default)
    }
}

impl<T: Deref> JsOption<T> {
    /// Converts from `&JsOption<T>` to `JsOption<&T::Target>`.
    pub fn as_deref(&self) -> JsOption<&<T as Deref>::Target> {
        self.as_ref().map(|val| val.deref())
    }
}

impl<T: DerefMut> JsOption<T> {
    /// Converts from `&mut JsOption<T>` to `JsOption<&mut T::Target>`.
    pub fn as_deref_mut(&mut self) -> JsOption<&mut <T as Deref>::Target> {
        self.as_mut().map(|val| val.deref_mut())
    }
}

impl<T> Default for JsOption<T> {
    /// Returns the default value, `JsOption::Undefined`.
    fn default() -> Self {
        Self::Undefined
    }
}

#![doc = include_str!("../README.md")]
#![deny(unreachable_pub)] // ! lib needs to check this item

/// types
pub mod types;

/// common
pub mod common;

/// model
pub mod model;

/// store
pub mod store;

/// Determine whether it needs serialization
#[inline]
#[allow(unused)]
fn is_empty_option_vec<T>(values: &Option<Vec<T>>) -> bool {
    !values.as_ref().is_some_and(|values| !values.is_empty())
}

//! functions for formatting data into text
#![warn(missing_docs, unreachable_pub, unused_crate_dependencies)]
#![deny(unused_must_use, rust_2018_idioms)]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
))]

/// number formatting
pub mod number_format;

/// binary formatting
pub mod binary_format;

/// string formatting
pub mod string_format;

/// exceptions
pub mod exceptions;

pub use binary_format::*;
pub use exceptions::*;
pub use number_format::*;
pub use string_format::*;

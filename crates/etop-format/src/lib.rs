//! functions for formatting data into text
#![warn(missing_docs, unreachable_pub, unused_crate_dependencies)]
#![deny(unused_must_use, rust_2018_idioms)]
#![doc(test(
    no_crate_inject,
    attr(deny(warnings, rust_2018_idioms), allow(dead_code, unused_variables))
))]

/// bool formatting
pub mod bool_format;

/// binary formatting
pub mod binary_format;

/// number formatting
pub mod number_format;

/// string formatting
pub mod string_format;

/// table formats
pub mod table_formats;

/// exceptions
pub mod exceptions;

pub use binary_format::*;
pub use bool_format::*;
pub use exceptions::*;
pub use number_format::*;
pub use string_format::*;
pub use table_formats::*;

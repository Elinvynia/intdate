//! [![ci-badge][]][ci] [![docs-badge][]][docs] [![crate-version]][crate-link]
//!
//! # intdate
//!
//! A simple integer date library inspired by Python's `int_date`.
//!
//! ## Sample Usage
//! ```rust
//!     let date = intdate::from_str("2020-01-01").unwrap();
//!     println!("{}", date.is_year_leap());
//! ```
//!
//! [ci]: https://github.com/Elinvynia/intdate/actions?query=workflow%3ARust
//! [ci-badge]: https://img.shields.io/github/workflow/status/Elinvynia/intdate/Rust/master?style=flat-square
//! [docs]: https://docs.rs/intdate
//! [docs-badge]: https://img.shields.io/badge/docs-online-5023dd.svg?style=flat-square
//! [crate-link]: https://crates.io/crates/intdate
//! [crate-version]: https://img.shields.io/crates/v/intdate.svg?style=flat-square

#![warn(missing_docs)]
#![warn(missing_debug_implementations)]

pub(crate) mod error;
pub(crate) mod helpers;
pub(crate) mod impls;

pub use error::IntDateError;
use helpers::*;
use std::convert::TryInto;
use std::str::FromStr;

/// Struct representing a valid intdate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct IntDate(u32);
impl IntDate {
    /// Returns the integer this date was made from.
    pub fn raw(&self) -> u32 {
        self.0
    }

    /// Returns the day.
    pub fn get_day(&self) -> u8 {
        get_day_part(self.0)
    }

    /// Returns the month.
    pub fn get_month(&self) -> u8 {
        get_month_part(self.0)
    }

    /// Returns the year.
    pub fn get_year(&self) -> u32 {
        get_year_part(self.0)
    }

    /// Returns if the year is leap or not.
    pub fn is_year_leap(&self) -> bool {
        is_year_leap(self.0)
    }
}

/// Helper function for creating an IntDate.
pub fn from_u32(input: u32) -> Result<IntDate, IntDateError> {
    input.try_into()
}

/// Helper function for creating an IntDate.
pub fn from_str<T: AsRef<str>>(input: T) -> Result<IntDate, IntDateError> {
    let input = input.as_ref();
    IntDate::from_str(input)
}

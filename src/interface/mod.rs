//! Module containing definitions related to network interfaces.
//!
//! This module provides the [`Interface`] struct and related enums and structs
//! such as [`Family`] and [`Mapping`]. These types are used to represent and
//! manipulate network interface configurations in an `interfaces(5)` file.
//!
//! Refer to the `interfaces(5)` manual page for details on the file format.

pub mod family;
pub mod mapping;
pub mod interface_struct;

pub use family::{Family, FamilyParseError};
pub use mapping::Mapping;
pub use interface_struct::Interface;

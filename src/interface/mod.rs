//! Module containing definitions related to network interfaces.
//!
//! This module provides the [`Interface`] struct and related enums and structs
//! such as [`Family`], [`Method`], [`Mapping`], [`InterfaceOption`], and the [`InterfaceBuilder`].
//! These types are used to represent and manipulate network interface configurations in an
//! `interfaces(5)` file.
//!
//! Refer to the `interfaces(5)` manual page for details on the file format.

pub mod family;
pub mod interface_builder;
pub mod interface_struct;
pub mod mapping;
pub mod method;
pub mod option;

pub use family::{Family, FamilyParseError};
pub use interface_builder::InterfaceBuilder;
pub use interface_struct::Interface;
pub use mapping::Mapping;
pub use method::Method;
pub use option::InterfaceOption;

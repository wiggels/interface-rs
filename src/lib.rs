//! # interface-rs
//!
//! A Rust library for parsing and manipulating an `interfaces(5)` file.
//!
//! This library provides structs and functions to load, parse, modify, and save
//! network interface configurations in the Debian-style `interfaces(5)` file
//! format. This file is typically found at `/etc/network/interfaces` on Debian-based
//! systems but may be located elsewhere depending on your system configuration.
//!
//! ## Features
//!
//! - **Load and parse** existing configurations from an `interfaces(5)` file.
//! - **Modify** interface configurations programmatically.
//! - **Save** changes back to the file system.
//!
//! ## Example
//!
//! ```rust
//! use interface_rs::NetworkInterfaces;
//! use interface_rs::interface::{Interface, Family};
//!
//! fn main() -> std::io::Result<()> {
//!     // Load the interfaces file
//!     let mut net_ifaces = NetworkInterfaces::load("/path/to/interfaces")?;
//!
//!     // Add a new interface
//!     let new_iface = Interface {
//!         name: "eth2".to_string(),
//!         auto: true,
//!         allow: vec!["hotplug".to_string()],
//!         family: Some(Family::Inet),
//!         method: Some("static".to_string()),
//!         options: vec![
//!             ("address".to_string(), "192.168.1.100".to_string()),
//!             ("netmask".to_string(), "255.255.255.0".to_string()),
//!         ],
//!         mapping: None,
//!     };
//!     net_ifaces.add_interface(new_iface);
//!
//!     // Save changes
//!     net_ifaces.save()?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## License
//!
//! This project is licensed under the MIT License.

pub mod interface;
pub mod network_interfaces;
mod parser;
pub mod error;

pub use network_interfaces::NetworkInterfaces;
pub use interface::{Interface, Family, Mapping};
pub use error::NetworkInterfacesError;

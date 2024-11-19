// src/lib.rs

//! # Interface-rs
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
//! - **Modify** network interface configurations programmatically.
//! - **Add or remove** network interfaces.
//! - **Save** changes back to the file system.
//! - **Fluent API** using the builder pattern for creating and modifying interfaces.
//!
//! ## Example
//!
//! ### Loading Interfaces and Modifying an Existing Interface
//!
//! ```rust
//! use interface_rs::NetworkInterfaces;
//! use interface_rs::interface::{Interface, Family};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Load interfaces (replace with an existing file path during tests)
//!     let mut net_ifaces = NetworkInterfaces::load("tests/interfaces")?;
//!
//!     // Retrieve and modify an existing interface
//!     if let Some(iface) = net_ifaces.get_interface("eth0") {
//!         let modified_iface = iface.edit()
//!             .with_method("static")
//!             .remove_option("address")
//!             .with_option("address", "192.168.1.50")
//!             .with_option("netmask", "255.255.255.0")
//!             .build();
//!         net_ifaces.add_interface(modified_iface);
//!     }
//!
//!     // Save changes
//!     net_ifaces.save()?;
//!     Ok(())
//! }
//! ```
//!
//! ### Adding a New Interface
//!
//! ```rust
//! use interface_rs::NetworkInterfaces;
//! use interface_rs::interface::{Interface, Family};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Load interfaces (replace with an existing file path during tests)
//!     let mut net_ifaces = NetworkInterfaces::load("tests/interfaces")?;
//!
//!     // Create a new interface using the builder pattern
//!     net_ifaces.add_interface(
//!         Interface::builder("swp1")
//!             .with_auto(true)
//!             .with_allow("hotplug")
//!             .with_family(Family::Inet)
//!             .with_method("static")
//!             .with_option("address", "192.168.100.1")
//!             .with_option("netmask", "255.255.255.0")
//!             .build()
//!     );
//!
//!     // Save changes back to the file
//!     net_ifaces.save()?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Deleting an Interface
//!
//! ```rust
//! use interface_rs::NetworkInterfaces;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Load interfaces (replace with an existing file path during tests)
//!     let mut net_ifaces = NetworkInterfaces::load("tests/interfaces")?;
//!
//!     // Delete an interface by name
//!     net_ifaces.delete_interface("eth0");
//!
//!     // Save changes back to the file
//!     net_ifaces.save()?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## License
//!
//! This project is licensed under the MIT License.

pub mod error;
pub mod interface;
pub mod network_interfaces;
mod parser;

pub use error::NetworkInterfacesError;
pub use interface::{Family, Interface, InterfaceBuilder, Mapping};
pub use network_interfaces::NetworkInterfaces;

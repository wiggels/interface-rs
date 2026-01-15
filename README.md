# interface-rs

[![Crates.io](https://img.shields.io/crates/v/interface-rs.svg)](https://crates.io/crates/interface-rs)
[![Documentation](https://docs.rs/interface-rs/badge.svg)](https://docs.rs/interface-rs)
[![License](https://img.shields.io/crates/l/interface-rs.svg)](https://github.com/wiggels/interface-rs/blob/main/LICENSE)

A Rust library for parsing and manipulating an `interfaces(5)` file.

This library provides structs and functions to load, parse, modify, and save network interface configurations in the Debian-style `interfaces(5)` file format. The `interfaces(5)` file is typically found at `/etc/network/interfaces` on Debian-based systems but may be located elsewhere depending on your system configuration.

---

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
  - [Loading Interfaces](#loading-interfaces)
  - [Retrieving an Interface](#retrieving-an-interface)
  - [Adding a New Interface](#adding-a-new-interface)
  - [Modifying an Existing Interface](#modifying-an-existing-interface)
  - [Deleting an Interface](#deleting-an-interface)
  - [Saving Changes](#saving-changes)
- [Documentation](#documentation)
- [Contributing](#contributing)
- [License](#license)
- [Acknowledgments](#acknowledgments)

---

## Features

- **Load and parse** existing configurations from an `interfaces(5)` file.
- **Modify** network interface configurations programmatically.
- **Add or remove** network interfaces.
- **Save** changes back to the file system.
- **Fluent API** using the builder pattern for creating and modifying interfaces.
- **Display** interfaces in the correct `interfaces(5)` file format.
- **Type-safe** enums for address families (`Family`) and configuration methods (`Method`).

---

## Installation

Add `interface-rs` to your `Cargo.toml`:

```toml
[dependencies]
interface-rs = "0.2"
```

Then run:

```shell
cargo build
```

---

## Usage

### Loading Interfaces

```rust
use interface_rs::NetworkInterfaces;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load the interfaces file
    let net_ifaces = NetworkInterfaces::load("/etc/network/interfaces")?;

    // Iterate over interfaces
    for (name, iface) in net_ifaces.iter() {
        println!("Found interface: {}", name);
    }

    Ok(())
}
```

### Retrieving an Interface

```rust
use interface_rs::NetworkInterfaces;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let net_ifaces = NetworkInterfaces::load("/etc/network/interfaces")?;

    // Get a specific interface
    if let Some(iface) = net_ifaces.get_interface("eth0") {
        println!("Interface {} found.", iface.name);
        println!("Auto: {}", iface.auto);
        println!("Method: {:?}", iface.method);
    } else {
        println!("Interface eth0 not found.");
    }

    Ok(())
}
```

### Adding a New Interface

```rust
use interface_rs::NetworkInterfaces;
use interface_rs::interface::{Interface, Family, Method};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut net_ifaces = NetworkInterfaces::load("/etc/network/interfaces")?;

    // Create a new interface using the builder pattern
    let new_iface = Interface::builder("swp1")
        .with_auto(true)
        .with_allow("hotplug")
        .with_family(Family::Inet)
        .with_method(Method::Static)
        .with_option("address", "192.168.100.1")
        .with_option("netmask", "255.255.255.0")
        .build();

    // Add the new interface to the collection
    net_ifaces.add_interface(new_iface);

    // Save changes back to the file
    net_ifaces.save()?;

    Ok(())
}
```

### Modifying an Existing Interface

```rust
use interface_rs::NetworkInterfaces;
use interface_rs::interface::{Interface, Family, Method};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut net_ifaces = NetworkInterfaces::load("/etc/network/interfaces")?;

    // Retrieve and modify an existing interface using the builder pattern
    if let Some(iface) = net_ifaces.get_interface("eth0") {
        let modified_iface = iface.edit()
            .with_method(Method::Static)
            .with_option("address", "192.168.1.50")
            .with_option("netmask", "255.255.255.0")
            .build();

        // Replace the existing interface with the modified one
        net_ifaces.add_interface(modified_iface);

        // Save changes back to the file
        net_ifaces.save()?;
    } else {
        println!("Interface eth0 not found.");
    }

    Ok(())
}
```

### Deleting an Interface

```rust
use interface_rs::NetworkInterfaces;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut net_ifaces = NetworkInterfaces::load("/etc/network/interfaces")?;

    // Delete an interface by name
    net_ifaces.delete_interface("eth0");

    // Save changes back to the file
    net_ifaces.save()?;

    println!("Interface eth0 has been deleted.");

    Ok(())
}
```

### Saving Changes

```rust
use interface_rs::NetworkInterfaces;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut net_ifaces = NetworkInterfaces::load("/etc/network/interfaces")?;

    // Make changes to interfaces...

    // Save changes back to the file
    net_ifaces.save()?;

    println!("Changes saved successfully.");

    Ok(())
}
```

---

## Supported Configuration

### Address Families

The library supports all standard address families defined in `interfaces(5)`:

- `Family::Inet` - IPv4 networking
- `Family::Inet6` - IPv6 networking
- `Family::IpX` - IPX networking
- `Family::Can` - CAN (Controller Area Network)

### Configuration Methods

Common methods are provided as enum variants for type safety:

- `Method::Static` - Static IP configuration
- `Method::Dhcp` - DHCP client configuration
- `Method::Loopback` - Loopback interface
- `Method::Manual` - Manual configuration
- `Method::Other(String)` - Any other method (e.g., `ppp`, `tunnel`, `bootp`)

---

## Documentation

For more detailed information, please refer to the [API documentation](https://docs.rs/interface-rs).

---

## Contributing

Contributions are welcome! If you'd like to contribute to `interface-rs`, please follow these steps:

1. **Fork** the repository on GitHub.
2. **Clone** your forked repository locally.
3. **Create** a new branch for your feature or bugfix.
4. **Commit** your changes with clear and descriptive messages.
5. **Push** your changes to your fork.
6. **Submit** a Pull Request to the `main` branch of the original repository.

Please ensure your code adheres to the project's coding standards and includes appropriate tests.

---

## License

This project is licensed under the MIT License. See the [LICENSE](https://github.com/wiggels/interface-rs/blob/main/LICENSE) file for details.

---

## Acknowledgments

- Inspired by the need for programmatically managing network interface configurations on Debian-based systems.
- Thanks to the Rust community for their excellent tools and documentation.

---

**Note**: This library is intended for use on systems that utilize the Debian-style `interfaces(5)` file format. It may not be compatible with other network configuration systems.

For any issues or feature requests, please open an issue on the [GitHub repository](https://github.com/wiggels/interface-rs).

---

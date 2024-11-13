# Interface-rs

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
  - [Modifying an Interface](#modifying-an-interface)
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
- **Display** interfaces in the correct `interfaces(5)` file format.

---

## Installation

Add `interface-rs` to your `Cargo.toml`:

```toml
[dependencies]
interface-rs = "0.1.0"
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

fn main() -> std::io::Result<()> {
    // Load the interfaces file
    let net_ifaces = NetworkInterfaces::load("/path/to/interfaces")?;

    // Iterate over interfaces
    for iface in net_ifaces.interfaces.values() {
        println!("Found interface: {}", iface.name);
    }

    Ok(())
}
```

### Retrieving an Interface

```rust
use interface_rs::NetworkInterfaces;

fn main() -> std::io::Result<()> {
    let net_ifaces = NetworkInterfaces::load("/path/to/interfaces")?;

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
use interface_rs::interface::{Interface, Family};

fn main() -> std::io::Result<()> {
    let mut net_ifaces = NetworkInterfaces::load("/path/to/interfaces")?;

    // Create a new interface
    let new_iface = Interface {
        name: "eth2".to_string(),
        auto: true,
        allow: vec!["hotplug".to_string()],
        family: Some(Family::Inet),
        method: Some("static".to_string()),
        options: vec![
            ("address".to_string(), "192.168.1.100".to_string()),
            ("netmask".to_string(), "255.255.255.0".to_string()),
        ],
        mapping: None,
    };

    // Add the new interface
    net_ifaces.add_interface(new_iface);

    // Save changes
    net_ifaces.save()?;

    Ok(())
}
```

### Modifying an Interface

```rust
use interface_rs::NetworkInterfaces;

fn main() -> std::io::Result<()> {
    let mut net_ifaces = NetworkInterfaces::load("/path/to/interfaces")?;

    // Modify an existing interface
    if let Some(iface) = net_ifaces.interfaces.get_mut("eth0") {
        iface.method = Some("static".to_string());
        iface.options.push(("address".to_string(), "192.168.1.50".to_string()));
        iface.options.push(("netmask".to_string(), "255.255.255.0".to_string()));
        println!("Interface eth0 modified.");
    } else {
        println!("Interface eth0 not found.");
    }

    // Save changes
    net_ifaces.save()?;

    Ok(())
}
```

### Saving Changes

```rust
use interface_rs::NetworkInterfaces;

fn main() -> std::io::Result<()> {
    let mut net_ifaces = NetworkInterfaces::load("/path/to/interfaces")?;

    // Make changes to interfaces...

    // Save changes back to the file
    net_ifaces.save()?;

    println!("Changes saved successfully.");

    Ok(())
}
```

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
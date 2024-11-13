use super::{Family, InterfaceBuilder, Mapping};
use std::{collections::HashMap, fmt};

/// Represents a network interface configuration in an `interfaces(5)` file.
///
/// The `Interface` struct encapsulates all the configuration details for a
/// network interface, including its name, whether it starts automatically,
/// allowed hotplug options, address family, method of configuration, and
/// additional options.
///
/// To construct an `Interface`, it is recommended to use the [`InterfaceBuilder`]
/// via the [`Interface::builder`] method for a more ergonomic and fluent API.
///
/// # Examples
///
/// Creating a new `Interface` using the builder pattern:
///
/// ```rust
/// use interface_rs::interface::{Interface, Family};
///
/// let iface = Interface::builder("eth0")
///     .with_auto(true)
///     .with_allow("hotplug")
///     .with_family(Family::Inet)
///     .with_method("dhcp")
///     .with_option("mtu", "1500")
///     .build();
/// ```
#[derive(Debug, Clone)]
pub struct Interface {
    /// The name of the interface (e.g., `"eth0"`).
    pub name: String,
    /// Indicates if the interface is set to start automatically.
    pub auto: bool,
    /// A list of `allow-*` directives associated with the interface.
    pub allow: Vec<String>,
    /// The address family (e.g., `inet`).
    pub family: Option<Family>,
    /// The method of configuration (e.g., `"static"`, `"dhcp"`).
    pub method: Option<String>,
    /// A list of options specified under the `iface` stanza.
    pub options: Vec<(String, String)>,
    /// Optional mapping configuration for the interface.
    pub mapping: Option<Mapping>,
}

impl Interface {
    /// Creates a new [`InterfaceBuilder`] for constructing an `Interface`.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the interface (e.g., `"eth0"`).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use interface_rs::interface::Interface;
    ///
    /// let builder = Interface::builder("eth0");
    /// ```
    pub fn builder(name: impl Into<String>) -> InterfaceBuilder {
        InterfaceBuilder::new(name)
    }

    /// Creates a new [`InterfaceBuilder`] initialized with this `Interface`'s data.
    ///
    /// This method allows you to modify an existing `Interface` using the builder pattern.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use interface_rs::interface::{Interface, Family};
    ///
    /// let iface = Interface::builder("eth0")
    ///     .with_auto(true)
    ///     .with_family(Family::Inet)
    ///     .with_method("dhcp")
    ///     .build();
    ///
    /// // Modify the existing interface
    /// let modified_iface = iface.edit()
    ///     .with_method("static")
    ///     .with_option("address", "192.168.1.50")
    ///     .build();
    /// ```
    pub fn edit(&self) -> InterfaceBuilder {
        InterfaceBuilder {
            name: self.name.clone(),
            auto: self.auto,
            allow: self.allow.clone(),
            family: self.family.clone(),
            method: self.method.clone(),
            options: self.options.iter().cloned().collect::<HashMap<_, _>>(),
            mapping: self.mapping.clone(),
        }
    }
}

impl fmt::Display for Interface {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.auto {
            writeln!(f, "auto {}", self.name)?;
        }
        for allow_type in &self.allow {
            writeln!(f, "allow-{} {}", allow_type, self.name)?;
        }
        if let Some(mapping) = &self.mapping {
            writeln!(f, "mapping {}", self.name)?;
            writeln!(f, "    script {}", mapping.script)?;
            for map in &mapping.maps {
                writeln!(f, "    map {}", map)?;
            }
        }
        if let (Some(family), Some(method)) = (&self.family, &self.method) {
            writeln!(f, "iface {} {} {}", self.name, family, method)?;
            for (option_name, option_value) in &self.options {
                writeln!(f, "    {} {}", option_name, option_value)?;
            }
        }
        Ok(())
    }
}

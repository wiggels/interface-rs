use super::{Family, InterfaceBuilder, Mapping, Method};
use std::fmt;

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
/// use interface_rs::interface::{Interface, Family, Method};
///
/// let iface = Interface::builder("eth0")
///     .with_auto(true)
///     .with_allow("hotplug")
///     .with_family(Family::Inet)
///     .with_method(Method::Dhcp)
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
    /// The method of configuration (e.g., `static`, `dhcp`).
    pub method: Option<Method>,
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
    /// use interface_rs::interface::{Interface, Family, Method};
    ///
    /// let iface = Interface::builder("eth0")
    ///     .with_auto(true)
    ///     .with_family(Family::Inet)
    ///     .with_method(Method::Dhcp)
    ///     .build();
    ///
    /// // Modify the existing interface
    /// let modified_iface = iface.edit()
    ///     .with_method(Method::Static)
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
            options: self.options.clone(),
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
            writeln!(f, "    script {}", mapping.script.display())?;
            for map in &mapping.maps {
                writeln!(f, "    map {}", map)?;
            }
        }
        write!(f, "iface {}", self.name)?;
        if let Some(family) = &self.family {
            write!(f, " {}", family)?;
        }
        if let Some(method) = &self.method {
            write!(f, " {}", method)?;
        }
        writeln!(f)?;
        // Sort options before printing
        let mut sorted_options = self.options.clone();
        sorted_options.sort_by(|a, b| a.0.cmp(&b.0));
        for (option_name, option_value) in &sorted_options {
            writeln!(f, "    {} {}", option_name, option_value)?;
        }
        Ok(())
    }
}

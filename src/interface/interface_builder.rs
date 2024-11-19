use super::{Family, Interface, Mapping};

/// A builder for constructing [`Interface`] instances.
///
/// The `InterfaceBuilder` struct provides a fluent API for building
/// `Interface` objects. It allows you to chain method calls to set various
/// fields, culminating in a `build()` method that constructs the `Interface`.
///
/// # Examples
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
pub struct InterfaceBuilder {
    pub(crate) name: String,
    pub(crate) auto: bool,
    pub(crate) allow: Vec<String>,
    pub(crate) family: Option<Family>,
    pub(crate) method: Option<String>,
    pub(crate) options: Vec<(String, String)>,
    pub(crate) mapping: Option<Mapping>,
}

impl InterfaceBuilder {
    /// Creates a new `InterfaceBuilder` with the specified interface name.
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
    pub fn new(name: impl Into<String>) -> Self {
        InterfaceBuilder {
            name: name.into(),
            auto: false,
            allow: Vec::new(),
            family: None,
            method: None,
            options: Vec::new(),
            mapping: None,
        }
    }

    /// Sets whether the interface should start automatically.
    ///
    /// # Arguments
    ///
    /// * `auto` - A boolean indicating if the interface should start automatically.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use interface_rs::interface::Interface;
    /// # let builder = Interface::builder("eth0");
    /// let builder = builder.with_auto(true);
    /// ```
    pub fn with_auto(mut self, auto: bool) -> Self {
        self.auto = auto;
        self
    }

    /// Adds an `allow-*` directive to the interface.
    ///
    /// # Arguments
    ///
    /// * `allow` - A string representing the allow directive (e.g., `"hotplug"`).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use interface_rs::interface::Interface;
    /// # let builder = Interface::builder("eth0");
    /// let builder = builder.with_allow("hotplug");
    /// ```
    pub fn with_allow(mut self, allow: impl Into<String>) -> Self {
        self.allow.push(allow.into());
        self
    }

    /// Sets the address family of the interface.
    ///
    /// # Arguments
    ///
    /// * `family` - The [`Family`] of the interface (e.g., `Family::Inet`).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use interface_rs::interface::{Interface, Family};
    /// let builder = Interface::builder("eth0")
    ///     .with_family(Family::Inet);
    /// ```
    pub fn with_family(mut self, family: Family) -> Self {
        self.family = Some(family);
        self
    }

    /// Sets the method of configuration for the interface.
    ///
    /// # Arguments
    ///
    /// * `method` - A string representing the method (e.g., `"static"`, `"dhcp"`).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use interface_rs::interface::Interface;
    /// let builder = Interface::builder("eth0")
    ///     .with_method("dhcp");
    /// ```
    pub fn with_method(mut self, method: impl Into<String>) -> Self {
        self.method = Some(method.into());
        self
    }

    /// Adds an option to the interface.
    ///
    /// # Arguments
    ///
    /// * `key` - The option name (e.g., `"address"`).
    /// * `value` - The option value (e.g., `"192.168.1.100"`).
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use interface_rs::interface::Interface;
    /// let builder = Interface::builder("eth0")
    ///     .with_option("address", "192.168.1.100");
    /// ```
    pub fn with_option(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.options.push((key.into(), value.into()));
        self
    }

    /// Sets the mapping configuration for the interface.
    ///
    /// # Arguments
    ///
    /// * `mapping` - A [`Mapping`] struct representing the mapping configuration.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use interface_rs::interface::{Interface, Mapping};
    /// let mapping = Mapping {
    ///     script: "/usr/local/bin/map-script".to_string(),
    ///     maps: vec!["eth0".to_string()],
    /// };
    /// let builder = Interface::builder("eth0")
    ///     .with_mapping(mapping);
    /// ```
    pub fn with_mapping(mut self, mapping: Mapping) -> Self {
        self.mapping = Some(mapping);
        self
    }

    /// Removes all options with the specified key from the interface configuration.
    ///
    /// This method removes all key-value pairs in the options where the key matches
    /// the specified `key`.
    ///
    /// # Arguments
    ///
    /// * `key` - The name of the option to remove (e.g., `"address"`).
    ///
    /// # Returns
    ///
    /// Returns the builder instance with the specified options removed, allowing
    /// further chained method calls.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use interface_rs::interface::Interface;
    /// let builder = Interface::builder("eth0")
    ///     .with_option("address", "192.168.1.100")
    ///     .with_option("address", "192.168.1.101")
    ///     .remove_option("address");
    ///
    /// // The builder no longer contains any "address" options.
    /// ```
    pub fn remove_option(mut self, key: &str) -> Self {
        self.options.retain(|(k, _)| k != key);
        self
    }

    /// Removes a specific option by its key and value from the interface configuration.
    ///
    /// This method removes only the key-value pair in the options where both the key
    /// matches the specified `key` and the value matches the specified `value`.
    ///
    /// # Arguments
    ///
    /// * `key` - The name of the option to remove (e.g., `"address"`).
    /// * `value` - The specific value of the option to remove (e.g., `"192.168.1.100"`).
    ///
    /// # Returns
    ///
    /// Returns the builder instance with the specified key-value pair removed,
    /// allowing further chained method calls.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use interface_rs::interface::Interface;
    /// let builder = Interface::builder("eth0")
    ///     .with_option("address", "192.168.1.100")
    ///     .with_option("address", "192.168.1.101")
    ///     .remove_option_value("address", "192.168.1.100");
    ///
    /// // The builder retains the option with the value "192.168.1.101" for "address",
    /// // but the pair ("address", "192.168.1.100") is removed.
    /// ```
    pub fn remove_option_value(mut self, key: &str, value: &str) -> Self {
        self.options.retain(|(k, v)| !(k == key && v == value));
        self
    }

    /// Builds the [`Interface`] instance.
    ///
    /// # Returns
    ///
    /// An `Interface` with the specified configuration.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use interface_rs::interface::{Interface, Family};
    /// let iface = Interface::builder("eth0")
    ///     .with_auto(true)
    ///     .with_family(Family::Inet)
    ///     .with_method("dhcp")
    ///     .build();
    /// ```
    pub fn build(self) -> Interface {
        Interface {
            name: self.name,
            auto: self.auto,
            allow: self.allow,
            family: self.family,
            method: self.method,
            options: self.options,
            mapping: self.mapping,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_remove_option() {
        let iface = Interface::builder("eth0")
            .with_option("address", "192.168.1.50")
            .with_option("netmask", "255.255.255.0")
            .with_option("address", "192.168.1.51") // Duplicate address key with different value
            .remove_option("address") // Should remove all "address" options
            .build();

        assert_eq!(iface.options.len(), 1);
        assert_eq!(
            iface.options[0],
            ("netmask".to_string(), "255.255.255.0".to_string())
        );
    }

    #[test]
    fn test_remove_option_value() {
        let iface = Interface::builder("eth0")
            .with_option("address", "192.168.1.50")
            .with_option("netmask", "255.255.255.0")
            .with_option("address", "192.168.1.51") // Duplicate address key with different value
            .with_option("address", "192.168.1.52") // Duplicate address key with different value
            .remove_option_value("address", "192.168.1.50") // Should remove only this address pair
            .build();

        assert_eq!(iface.options.len(), 3);
        assert!(iface
            .options
            .contains(&("netmask".to_string(), "255.255.255.0".to_string())));
        assert!(iface
            .options
            .contains(&("address".to_string(), "192.168.1.51".to_string())));
        assert!(iface
            .options
            .contains(&("address".to_string(), "192.168.1.52".to_string())));
    }
}

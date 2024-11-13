use super::{Family, Mapping};
use std::fmt;

/// Represents a network interface configuration in an `interfaces(5)` file.
///
/// The `Interface` struct encapsulates all the configuration details for a
/// network interface, including its name, whether it starts automatically,
/// allowed hotplug options, address family, method of configuration, and
/// additional options.
///
/// # Examples
///
/// Creating a new `Interface`:
///
/// ```rust
/// use interface_rs::interface::{Interface, Family};
///
/// let iface = Interface {
///     name: "eth0".to_string(),
///     auto: true,
///     allow: vec!["hotplug".to_string()],
///     family: Some(Family::Inet),
///     method: Some("dhcp".to_string()),
///     options: vec![],
///     mapping: None,
/// };
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

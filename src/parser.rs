use crate::interface::{Family, Interface};
use crate::error::ParserError;
use std::collections::HashMap;

/// A parser for an `interfaces(5)` file.
///
/// The `Parser` struct provides methods to parse the content of the interfaces file
/// and produce a collection of `Interface` instances.
pub struct Parser;

impl Parser {
    /// Creates a new `Parser` instance.
    pub fn new() -> Self {
        Parser
    }

    /// Parses the content of the interfaces file.
    ///
    /// # Arguments
    ///
    /// * `content` - A string slice containing the file content.
    ///
    /// # Returns
    ///
    /// A `Result` containing a `HashMap<String, Interface>` if successful,
    /// or a `ParserError` if parsing fails.
    pub fn parse(
        &self,
        content: &str,
    ) -> Result<HashMap<String, Interface>, ParserError> {
        let mut interfaces = HashMap::new();
        let mut lines = content.lines().enumerate().peekable();
        let mut current_interface: Option<Interface> = None;

        while let Some((line_number, line)) = lines.next() {
            let line = line.trim();

            // Skip empty lines and comments
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let tokens: Vec<&str> = line.split_whitespace().collect();
            if tokens.is_empty() {
                continue;
            }

            // Finish the previous interface if necessary
            match tokens[0] {
                "auto" | "mapping" | "iface" => {
                    if let Some(iface) = current_interface.take() {
                        interfaces.insert(iface.name.clone(), iface);
                    }
                }
                s if s.starts_with("allow-") => {
                    if let Some(iface) = current_interface.take() {
                        interfaces.insert(iface.name.clone(), iface);
                    }
                }
                _ => {}
            }

            match tokens[0] {
                "auto" => {
                    for &iface_name in &tokens[1..] {
                        if let Some(iface) = interfaces.get_mut(iface_name) {
                            // If interface exists, set auto to true
                            iface.auto = true;
                        } else {
                            // Interface doesn't exist yet, create it with auto = true
                            interfaces.insert(
                                iface_name.to_string(),
                                Interface::builder(iface_name).with_auto(true).build(),
                            );
                        }
                    }
                }
                s if s.starts_with("allow-") => {
                    let allow_type = s.strip_prefix("allow-").unwrap();
                    for &iface_name in &tokens[1..] {
                        if let Some(iface) = interfaces.get_mut(iface_name) {
                            // If interface exists, add to allow list
                            iface.allow.push(allow_type.to_string());
                        } else {
                            // Interface doesn't exist yet, create it with allow
                            let mut iface = Interface::builder(iface_name).build();
                            iface.allow.push(allow_type.to_string());
                            interfaces.insert(iface_name.to_string(), iface);
                        }
                    }
                }
                "iface" => {
                    // Start a new interface
                    let iface_name = tokens.get(1).ok_or_else(|| ParserError {
                        message: "Missing interface name in 'iface' stanza".to_string(),
                        line: Some(line_number + 1),
                    })?.to_string();

                    // Remove existing interface if any
                    let existing_iface = interfaces.remove(&iface_name);

                    // Build the interface using existing settings if available
                    let mut builder = if let Some(existing_iface) = existing_iface {
                        existing_iface.edit()
                    } else {
                        Interface::builder(iface_name.clone())
                    };

                    // Parse family
                    let family = match tokens.get(2) {
                        Some(s) => Some(s.parse::<Family>().map_err(|e| ParserError {
                            message: e.to_string(),
                            line: Some(line_number + 1),
                        })?),
                        None => None,
                    };

                    // Parse method
                    let method = tokens.get(3).map(|s| s.to_string());

                    if let Some(family) = family {
                        builder = builder.with_family(family);
                    }

                    if let Some(method) = method {
                        builder = builder.with_method(method);
                    }

                    current_interface = Some(builder.build());
                }
                "mapping" => {
                    // Handle 'mapping' stanzas if needed
                    // For now, we ignore unknown stanzas
                }
                _ => {
                    // Parse options under 'iface' stanza
                    if let Some(iface) = &mut current_interface {
                        let mut tokens = line.split_whitespace();
                        if let Some(option_name) = tokens.next() {
                            let option_value = tokens.collect::<Vec<&str>>().join(" ");
                            iface.options.push((
                                option_name.to_string(),
                                option_value,
                            ));
                        }
                    } else {
                        // Handle global options if needed
                        // For now, we ignore unknown stanzas outside of an 'iface'
                    }
                }
            }
        }

        // Insert the last interface
        if let Some(iface) = current_interface {
            interfaces.insert(iface.name.clone(), iface);
        }

        Ok(interfaces)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interface::Family;

    #[test]
    fn test_parse_iface_without_family_and_method() {
        let content = r#"
auto eth0
iface eth0
    address 10.130.17.36/255.255.255.128
    vrf mgmt
"#;
        let parser = Parser::new();
        let interfaces = parser.parse(content).unwrap();
        assert!(interfaces.contains_key("eth0"));
        let iface = &interfaces["eth0"];
        assert_eq!(iface.name, "eth0");
        assert_eq!(iface.family, None);
        assert_eq!(iface.method, None);
        assert!(iface.options.contains(&("address".to_string(), "10.130.17.36/255.255.255.128".to_string())));
        assert!(iface.options.contains(&("vrf".to_string(), "mgmt".to_string())));
    }

    #[test]
    fn test_parse_iface_with_family_and_method() {
        let content = r#"
iface eth1 inet static
    address 192.168.1.10
    netmask 255.255.255.0
"#;
        let parser = Parser::new();
        let interfaces = parser.parse(content).unwrap();
        assert!(interfaces.contains_key("eth1"));
        let iface = &interfaces["eth1"];
        assert_eq!(iface.name, "eth1");
        assert_eq!(iface.family, Some(Family::Inet));
        assert_eq!(iface.method.as_deref(), Some("static"));
        assert!(iface.options.contains(&("address".to_string(), "192.168.1.10".to_string())));
        assert!(iface.options.contains(&("netmask".to_string(), "255.255.255.0".to_string())));
    }

    #[test]
    fn test_parse_multiple_interfaces() {
        let content = r#"
auto lo
iface lo inet loopback

auto eth0
iface eth0 inet dhcp

auto wlan0
iface wlan0 inet static
    address 192.168.0.100
    netmask 255.255.255.0
"#;
        let parser = Parser::new();
        let interfaces = parser.parse(content).unwrap();

        assert_eq!(interfaces.len(), 3);

        // Check 'lo' interface
        let lo_iface = &interfaces["lo"];
        assert_eq!(lo_iface.name, "lo");
        assert_eq!(lo_iface.auto, true);
        assert_eq!(lo_iface.family, Some(Family::Inet));
        assert_eq!(lo_iface.method.as_deref(), Some("loopback"));

        // Check 'eth0' interface
        let eth0_iface = &interfaces["eth0"];
        assert_eq!(eth0_iface.name, "eth0");
        assert_eq!(eth0_iface.auto, true);
        assert_eq!(eth0_iface.family, Some(Family::Inet));
        assert_eq!(eth0_iface.method.as_deref(), Some("dhcp"));

        // Check 'wlan0' interface
        let wlan0_iface = &interfaces["wlan0"];
        assert_eq!(wlan0_iface.name, "wlan0");
        assert_eq!(wlan0_iface.auto, true);
        assert_eq!(wlan0_iface.family, Some(Family::Inet));
        assert_eq!(wlan0_iface.method.as_deref(), Some("static"));
        assert!(wlan0_iface.options.contains(&("address".to_string(), "192.168.0.100".to_string())));
        assert!(wlan0_iface.options.contains(&("netmask".to_string(), "255.255.255.0".to_string())));
    }

    #[test]
    fn test_parse_multiple_interfaces_strange_order() {
        let content = r#"
iface lo inet loopback
iface eth0 inet dhcp
auto eth0

auto wlan0
auto lo
iface wlan0 inet static
    address 192.168.0.100
    netmask 255.255.255.0
"#;
        let parser = Parser::new();
        let interfaces = parser.parse(content).unwrap();

        assert_eq!(interfaces.len(), 3);

        // Check 'lo' interface
        let lo_iface = &interfaces["lo"];
        assert_eq!(lo_iface.name, "lo");
        assert_eq!(lo_iface.auto, true);
        assert_eq!(lo_iface.family, Some(Family::Inet));
        assert_eq!(lo_iface.method.as_deref(), Some("loopback"));

        // Check 'eth0' interface
        let eth0_iface = &interfaces["eth0"];
        assert_eq!(eth0_iface.name, "eth0");
        assert_eq!(eth0_iface.auto, true);
        assert_eq!(eth0_iface.family, Some(Family::Inet));
        assert_eq!(eth0_iface.method.as_deref(), Some("dhcp"));

        // Check 'wlan0' interface
        let wlan0_iface = &interfaces["wlan0"];
        assert_eq!(wlan0_iface.name, "wlan0");
        assert_eq!(wlan0_iface.auto, true);
        assert_eq!(wlan0_iface.family, Some(Family::Inet));
        assert_eq!(wlan0_iface.method.as_deref(), Some("static"));
        assert!(wlan0_iface.options.contains(&("address".to_string(), "192.168.0.100".to_string())));
        assert!(wlan0_iface.options.contains(&("netmask".to_string(), "255.255.255.0".to_string())));
    }
}

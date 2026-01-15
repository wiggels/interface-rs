use crate::error::ParserError;
use crate::interface::{Family, Interface, Method};
use std::collections::HashMap;

/// A parser for an `interfaces(5)` file.
///
/// The `Parser` struct provides methods to parse the content of the interfaces file
/// and produce a collection of `Interface` instances.
pub struct Parser;

type ParseResult = Result<(HashMap<String, Interface>, Vec<String>, Vec<String>), ParserError>;

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
    /// A `Result` containing a tuple `(interfaces, comments, sources)` if successful,
    /// or a `ParserError` if parsing fails.
    pub fn parse(&self, content: &str) -> ParseResult {
        let mut interfaces = HashMap::new();
        let lines = content.lines().enumerate().peekable();
        let mut current_interface: Option<Interface> = None;
        let mut comments = Vec::new();
        let mut sources = Vec::new();

        for (line_number, line) in lines {
            let line = line.trim();

            // Collect comments at the top
            if line.starts_with('#') {
                if interfaces.is_empty() && current_interface.is_none() {
                    comments.push(line.to_string());
                }
                continue;
            }

            // Collect source directives
            if line.starts_with("source") {
                sources.push(line.to_string());
                continue;
            }

            // Skip empty lines
            if line.is_empty() {
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
                    let iface_name = tokens
                        .get(1)
                        .ok_or_else(|| ParserError {
                            message: "Missing interface name in 'iface' stanza".to_string(),
                            line: Some(line_number + 1),
                        })?
                        .to_string();

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
                        Some(s) => s.parse::<Family>().ok(),
                        None => None,
                    };

                    // Parse method
                    let method: Option<Method> = match tokens.len() {
                        // If family is valid, method is the next token
                        4 if family.is_some() => Some(tokens[3].parse().unwrap()),
                        // If family is absent, interpret the third token as the method
                        3 if family.is_none() => Some(tokens[2].parse().unwrap()),
                        _ => None,
                    };

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
                            iface.options.push((option_name.to_string(), option_value));
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

        Ok((interfaces, comments, sources))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::interface::{Family, Method};

    #[test]
    fn test_parse_iface_without_family_and_method() {
        let content = r#"
auto eth0
iface eth0
    address 10.130.17.36/255.255.255.128
    vrf mgmt
"#;
        let parser = Parser::new();
        let (interfaces, _comments, _sources) = parser.parse(content).unwrap();
        assert!(interfaces.contains_key("eth0"));
        let iface = &interfaces["eth0"];
        assert_eq!(iface.name, "eth0");
        assert_eq!(iface.family, None);
        assert_eq!(iface.method, None);
        assert!(iface.options.contains(&(
            "address".to_string(),
            "10.130.17.36/255.255.255.128".to_string()
        )));
        assert!(iface
            .options
            .contains(&("vrf".to_string(), "mgmt".to_string())));
    }

    #[test]
    fn test_parse_iface_with_family_and_method() {
        let content = r#"
iface eth1 inet static
    address 192.168.1.10
    netmask 255.255.255.0
"#;
        let parser = Parser::new();
        let (interfaces, _comments, _sources) = parser.parse(content).unwrap();
        assert!(interfaces.contains_key("eth1"));
        let iface = &interfaces["eth1"];
        assert_eq!(iface.name, "eth1");
        assert_eq!(iface.family, Some(Family::Inet));
        assert_eq!(iface.method, Some(Method::Static));
        assert!(iface
            .options
            .contains(&("address".to_string(), "192.168.1.10".to_string())));
        assert!(iface
            .options
            .contains(&("netmask".to_string(), "255.255.255.0".to_string())));
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
        let (interfaces, _comments, _sources) = parser.parse(content).unwrap();

        assert_eq!(interfaces.len(), 3);

        // Check 'lo' interface
        let lo_iface = &interfaces["lo"];
        assert_eq!(lo_iface.name, "lo");
        assert!(lo_iface.auto);
        assert_eq!(lo_iface.family, Some(Family::Inet));
        assert_eq!(lo_iface.method, Some(Method::Loopback));

        // Check 'eth0' interface
        let eth0_iface = &interfaces["eth0"];
        assert_eq!(eth0_iface.name, "eth0");
        assert!(eth0_iface.auto);
        assert_eq!(eth0_iface.family, Some(Family::Inet));
        assert_eq!(eth0_iface.method, Some(Method::Dhcp));

        // Check 'wlan0' interface
        let wlan0_iface = &interfaces["wlan0"];
        assert_eq!(wlan0_iface.name, "wlan0");
        assert!(wlan0_iface.auto);
        assert_eq!(wlan0_iface.family, Some(Family::Inet));
        assert_eq!(wlan0_iface.method, Some(Method::Static));
        assert!(wlan0_iface
            .options
            .contains(&("address".to_string(), "192.168.0.100".to_string())));
        assert!(wlan0_iface
            .options
            .contains(&("netmask".to_string(), "255.255.255.0".to_string())));
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
        let (interfaces, _comments, _sources) = parser.parse(content).unwrap();

        assert_eq!(interfaces.len(), 3);

        // Check 'lo' interface
        let lo_iface = &interfaces["lo"];
        assert_eq!(lo_iface.name, "lo");
        assert!(lo_iface.auto);
        assert_eq!(lo_iface.family, Some(Family::Inet));
        assert_eq!(lo_iface.method, Some(Method::Loopback));

        // Check 'eth0' interface
        let eth0_iface = &interfaces["eth0"];
        assert_eq!(eth0_iface.name, "eth0");
        assert!(eth0_iface.auto);
        assert_eq!(eth0_iface.family, Some(Family::Inet));
        assert_eq!(eth0_iface.method, Some(Method::Dhcp));

        // Check 'wlan0' interface
        let wlan0_iface = &interfaces["wlan0"];
        assert_eq!(wlan0_iface.name, "wlan0");
        assert!(wlan0_iface.auto);
        assert_eq!(wlan0_iface.family, Some(Family::Inet));
        assert_eq!(wlan0_iface.method, Some(Method::Static));
        assert!(wlan0_iface
            .options
            .contains(&("address".to_string(), "192.168.0.100".to_string())));
        assert!(wlan0_iface
            .options
            .contains(&("netmask".to_string(), "255.255.255.0".to_string())));
    }

    #[test]
    fn test_parse_multiple_interfaces_cumulus() {
        let content = r#"
auto swp54
iface swp54
    bridge-access 199
    mstpctl-bpduguard yes
    mstpctl-portadminedge yes
    mtu 9216
    post-down /some/script.sh
    post-up /some/script.sh

auto bridge
iface bridge
    bridge-ports swp1 swp2 swp3 swp4 swp5 swp6 swp7 swp8 swp9 swp10 swp11 swp12 swp13 swp14 swp15 swp16 swp17 swp18 swp19 swp20 swp21 swp22 swp23 swp24 swp31 swp32 swp33 swp34 swp35 swp36 swp37 swp38 swp39 swp40 swp41 swp42 swp43 swp44 swp45 swp46 swp47 swp48 swp49 swp50 swp51 swp52 swp53 swp54
    bridge-pvid 1
    bridge-vids 100-154 199
    bridge-vlan-aware yes

auto mgmt
iface mgmt
    address 127.0.0.1/8
    address ::1/128
    vrf-table auto

auto vlan101
iface vlan101
    mtu 9216
    post-up /some/script.sh
    vlan-id 101
    vlan-raw-device bridge
    "#;
        let parser = Parser::new();
        let (interfaces, _comments, _sources) = parser.parse(content).unwrap();

        assert_eq!(interfaces.len(), 4);

        // Check 'swp54' interface
        let swp54_iface = &interfaces["swp54"];
        assert_eq!(swp54_iface.name, "swp54");
        assert_eq!(swp54_iface.auto, true);
        assert_eq!(swp54_iface.family, None);
        assert_eq!(swp54_iface.method, None);
        // Check options
        assert!(swp54_iface
            .options
            .contains(&("bridge-access".to_string(), "199".to_string())));
        assert!(swp54_iface
            .options
            .contains(&("mstpctl-bpduguard".to_string(), "yes".to_string())));
        assert!(swp54_iface
            .options
            .contains(&("mstpctl-portadminedge".to_string(), "yes".to_string())));
        assert!(swp54_iface
            .options
            .contains(&("mtu".to_string(), "9216".to_string())));
        assert!(swp54_iface
            .options
            .contains(&("post-down".to_string(), "/some/script.sh".to_string())));
        assert!(swp54_iface
            .options
            .contains(&("post-up".to_string(), "/some/script.sh".to_string())));

        // Check 'bridge' interface
        let bridge_iface = &interfaces["bridge"];
        assert_eq!(bridge_iface.name, "bridge");
        assert_eq!(bridge_iface.auto, true);
        assert_eq!(bridge_iface.family, None);
        assert_eq!(bridge_iface.method, None);
        // Check options
        assert!(bridge_iface.options.contains(&("bridge-ports".to_string(), "swp1 swp2 swp3 swp4 swp5 swp6 swp7 swp8 swp9 swp10 swp11 swp12 swp13 swp14 swp15 swp16 swp17 swp18 swp19 swp20 swp21 swp22 swp23 swp24 swp31 swp32 swp33 swp34 swp35 swp36 swp37 swp38 swp39 swp40 swp41 swp42 swp43 swp44 swp45 swp46 swp47 swp48 swp49 swp50 swp51 swp52 swp53 swp54".to_string())));
        assert!(bridge_iface
            .options
            .contains(&("bridge-pvid".to_string(), "1".to_string())));
        assert!(bridge_iface
            .options
            .contains(&("bridge-vids".to_string(), "100-154 199".to_string())));
        assert!(bridge_iface
            .options
            .contains(&("bridge-vlan-aware".to_string(), "yes".to_string())));

        // Check 'mgmt' interface
        let mgmt_iface = &interfaces["mgmt"];
        assert_eq!(mgmt_iface.name, "mgmt");
        assert_eq!(mgmt_iface.auto, true);
        assert_eq!(mgmt_iface.family, None);
        assert_eq!(mgmt_iface.method, None);
        // Check options
        assert!(mgmt_iface
            .options
            .contains(&("address".to_string(), "127.0.0.1/8".to_string())));
        assert!(mgmt_iface
            .options
            .contains(&("address".to_string(), "::1/128".to_string())));
        assert!(mgmt_iface
            .options
            .contains(&("vrf-table".to_string(), "auto".to_string())));

        // Check 'vlan101' interface
        let vlan101_iface = &interfaces["vlan101"];
        assert_eq!(vlan101_iface.name, "vlan101");
        assert_eq!(vlan101_iface.auto, true);
        assert_eq!(vlan101_iface.family, None);
        assert_eq!(vlan101_iface.method, None);
        // Check options
        assert!(vlan101_iface
            .options
            .contains(&("mtu".to_string(), "9216".to_string())));
        assert!(vlan101_iface
            .options
            .contains(&("post-up".to_string(), "/some/script.sh".to_string())));
        assert!(vlan101_iface
            .options
            .contains(&("vlan-id".to_string(), "101".to_string())));
        assert!(vlan101_iface
            .options
            .contains(&("vlan-raw-device".to_string(), "bridge".to_string())));

        // Check print/display formatting
        // At the end of the test
        let mut output = String::new();
        output.push_str(&format!("{}\n", swp54_iface));
        output.push_str(&format!("{}\n", bridge_iface));
        output.push_str(&format!("{}\n", mgmt_iface));
        output.push_str(&format!("{}\n", vlan101_iface));

        // Expected output
        let expected_output = r#"
auto swp54
iface swp54
    bridge-access 199
    mstpctl-bpduguard yes
    mstpctl-portadminedge yes
    mtu 9216
    post-down /some/script.sh
    post-up /some/script.sh

auto bridge
iface bridge
    bridge-ports swp1 swp2 swp3 swp4 swp5 swp6 swp7 swp8 swp9 swp10 swp11 swp12 swp13 swp14 swp15 swp16 swp17 swp18 swp19 swp20 swp21 swp22 swp23 swp24 swp31 swp32 swp33 swp34 swp35 swp36 swp37 swp38 swp39 swp40 swp41 swp42 swp43 swp44 swp45 swp46 swp47 swp48 swp49 swp50 swp51 swp52 swp53 swp54
    bridge-pvid 1
    bridge-vids 100-154 199
    bridge-vlan-aware yes

auto mgmt
iface mgmt
    address 127.0.0.1/8
    address ::1/128
    vrf-table auto

auto vlan101
iface vlan101
    mtu 9216
    post-up /some/script.sh
    vlan-id 101
    vlan-raw-device bridge
"#;

        // Remove extra blank lines if any
        let expected_output = expected_output.trim();
        let output = output.trim();

        assert_eq!(output, expected_output);
    }
}

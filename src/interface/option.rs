//! Strongly typed interface options for `interfaces(5)` configuration.
//!
//! This module provides the [`InterfaceOption`] enum which represents all known
//! interface configuration options with proper typing.

use std::fmt;

/// Represents a network interface configuration option with proper typing.
///
/// This enum provides strongly typed variants for common interface options,
/// ensuring type safety and validation at compile time where possible.
///
/// Unknown options are captured in the [`InterfaceOption::Other`] variant.
#[derive(Debug, Clone, PartialEq)]
pub enum InterfaceOption {
    /// IP address, optionally with CIDR notation (e.g., "192.168.1.100" or "192.168.1.100/24")
    Address(String),
    /// Network mask (e.g., "255.255.255.0")
    Netmask(String),
    /// Default gateway address
    Gateway(String),
    /// Broadcast address
    Broadcast(String),
    /// Network address
    Network(String),
    /// Maximum Transmission Unit
    Mtu(u16),
    /// VLAN ID for bridge access port
    BridgeAccess(u16),
    /// List of bridge member ports
    BridgePorts(Vec<String>),
    /// Bridge Port VLAN ID (PVID)
    BridgePvid(u16),
    /// Bridge VLAN IDs (can be ranges like "100-154 199")
    BridgeVids(String),
    /// Whether the bridge is VLAN-aware
    BridgeVlanAware(bool),
    /// MSTP BPDU guard setting
    MstpctlBpduguard(bool),
    /// MSTP port admin edge setting
    MstpctlPortadminedge(bool),
    /// Script to run after interface comes up
    PostUp(String),
    /// Script to run before interface goes down
    PreDown(String),
    /// Script to run after interface goes down
    PostDown(String),
    /// Script to run before interface comes up
    PreUp(String),
    /// VRF name
    Vrf(String),
    /// VRF table (can be "auto" or a number)
    VrfTable(String),
    /// VLAN ID
    VlanId(u16),
    /// Raw device for VLAN
    VlanRawDevice(String),
    /// Hardware address (MAC)
    HwAddress(String),
    /// DNS nameservers
    DnsNameservers(String),
    /// DNS search domains
    DnsSearch(String),
    /// Metric for the route
    Metric(u32),
    /// Point-to-point address
    Pointopoint(String),
    /// Media type
    Media(String),
    /// Any other option not explicitly defined
    Other(String, String),
}

impl InterfaceOption {
    /// Returns the option name as it appears in the interfaces file.
    pub fn name(&self) -> &str {
        match self {
            InterfaceOption::Address(_) => "address",
            InterfaceOption::Netmask(_) => "netmask",
            InterfaceOption::Gateway(_) => "gateway",
            InterfaceOption::Broadcast(_) => "broadcast",
            InterfaceOption::Network(_) => "network",
            InterfaceOption::Mtu(_) => "mtu",
            InterfaceOption::BridgeAccess(_) => "bridge-access",
            InterfaceOption::BridgePorts(_) => "bridge-ports",
            InterfaceOption::BridgePvid(_) => "bridge-pvid",
            InterfaceOption::BridgeVids(_) => "bridge-vids",
            InterfaceOption::BridgeVlanAware(_) => "bridge-vlan-aware",
            InterfaceOption::MstpctlBpduguard(_) => "mstpctl-bpduguard",
            InterfaceOption::MstpctlPortadminedge(_) => "mstpctl-portadminedge",
            InterfaceOption::PostUp(_) => "post-up",
            InterfaceOption::PreDown(_) => "pre-down",
            InterfaceOption::PostDown(_) => "post-down",
            InterfaceOption::PreUp(_) => "pre-up",
            InterfaceOption::Vrf(_) => "vrf",
            InterfaceOption::VrfTable(_) => "vrf-table",
            InterfaceOption::VlanId(_) => "vlan-id",
            InterfaceOption::VlanRawDevice(_) => "vlan-raw-device",
            InterfaceOption::HwAddress(_) => "hwaddress",
            InterfaceOption::DnsNameservers(_) => "dns-nameservers",
            InterfaceOption::DnsSearch(_) => "dns-search",
            InterfaceOption::Metric(_) => "metric",
            InterfaceOption::Pointopoint(_) => "pointopoint",
            InterfaceOption::Media(_) => "media",
            InterfaceOption::Other(name, _) => name,
        }
    }

    /// Returns the option value as a string.
    pub fn value(&self) -> String {
        match self {
            InterfaceOption::Address(v) => v.clone(),
            InterfaceOption::Netmask(v) => v.clone(),
            InterfaceOption::Gateway(v) => v.clone(),
            InterfaceOption::Broadcast(v) => v.clone(),
            InterfaceOption::Network(v) => v.clone(),
            InterfaceOption::Mtu(v) => v.to_string(),
            InterfaceOption::BridgeAccess(v) => v.to_string(),
            InterfaceOption::BridgePorts(v) => v.join(" "),
            InterfaceOption::BridgePvid(v) => v.to_string(),
            InterfaceOption::BridgeVids(v) => v.clone(),
            InterfaceOption::BridgeVlanAware(v) => if *v { "yes" } else { "no" }.to_string(),
            InterfaceOption::MstpctlBpduguard(v) => if *v { "yes" } else { "no" }.to_string(),
            InterfaceOption::MstpctlPortadminedge(v) => if *v { "yes" } else { "no" }.to_string(),
            InterfaceOption::PostUp(v) => v.clone(),
            InterfaceOption::PreDown(v) => v.clone(),
            InterfaceOption::PostDown(v) => v.clone(),
            InterfaceOption::PreUp(v) => v.clone(),
            InterfaceOption::Vrf(v) => v.clone(),
            InterfaceOption::VrfTable(v) => v.clone(),
            InterfaceOption::VlanId(v) => v.to_string(),
            InterfaceOption::VlanRawDevice(v) => v.clone(),
            InterfaceOption::HwAddress(v) => v.clone(),
            InterfaceOption::DnsNameservers(v) => v.clone(),
            InterfaceOption::DnsSearch(v) => v.clone(),
            InterfaceOption::Metric(v) => v.to_string(),
            InterfaceOption::Pointopoint(v) => v.clone(),
            InterfaceOption::Media(v) => v.clone(),
            InterfaceOption::Other(_, v) => v.clone(),
        }
    }

    /// Creates an InterfaceOption from a key-value pair.
    ///
    /// This parses the value into the appropriate type based on the key.
    pub fn from_key_value(key: &str, value: &str) -> Self {
        match key {
            "address" => InterfaceOption::Address(value.to_string()),
            "netmask" => InterfaceOption::Netmask(value.to_string()),
            "gateway" => InterfaceOption::Gateway(value.to_string()),
            "broadcast" => InterfaceOption::Broadcast(value.to_string()),
            "network" => InterfaceOption::Network(value.to_string()),
            "mtu" => value
                .parse()
                .map(InterfaceOption::Mtu)
                .unwrap_or_else(|_| InterfaceOption::Other(key.to_string(), value.to_string())),
            "bridge-access" => value
                .parse()
                .map(InterfaceOption::BridgeAccess)
                .unwrap_or_else(|_| InterfaceOption::Other(key.to_string(), value.to_string())),
            "bridge-ports" => {
                InterfaceOption::BridgePorts(value.split_whitespace().map(String::from).collect())
            }
            "bridge-pvid" => value
                .parse()
                .map(InterfaceOption::BridgePvid)
                .unwrap_or_else(|_| InterfaceOption::Other(key.to_string(), value.to_string())),
            "bridge-vids" => InterfaceOption::BridgeVids(value.to_string()),
            "bridge-vlan-aware" => InterfaceOption::BridgeVlanAware(parse_bool(value)),
            "mstpctl-bpduguard" => InterfaceOption::MstpctlBpduguard(parse_bool(value)),
            "mstpctl-portadminedge" => InterfaceOption::MstpctlPortadminedge(parse_bool(value)),
            "post-up" => InterfaceOption::PostUp(value.to_string()),
            "pre-down" => InterfaceOption::PreDown(value.to_string()),
            "post-down" => InterfaceOption::PostDown(value.to_string()),
            "pre-up" => InterfaceOption::PreUp(value.to_string()),
            "vrf" => InterfaceOption::Vrf(value.to_string()),
            "vrf-table" => InterfaceOption::VrfTable(value.to_string()),
            "vlan-id" => value
                .parse()
                .map(InterfaceOption::VlanId)
                .unwrap_or_else(|_| InterfaceOption::Other(key.to_string(), value.to_string())),
            "vlan-raw-device" => InterfaceOption::VlanRawDevice(value.to_string()),
            "hwaddress" => InterfaceOption::HwAddress(value.to_string()),
            "dns-nameservers" => InterfaceOption::DnsNameservers(value.to_string()),
            "dns-search" => InterfaceOption::DnsSearch(value.to_string()),
            "metric" => value
                .parse()
                .map(InterfaceOption::Metric)
                .unwrap_or_else(|_| InterfaceOption::Other(key.to_string(), value.to_string())),
            "pointopoint" => InterfaceOption::Pointopoint(value.to_string()),
            "media" => InterfaceOption::Media(value.to_string()),
            _ => InterfaceOption::Other(key.to_string(), value.to_string()),
        }
    }

    /// Converts the option back to a key-value tuple.
    pub fn to_key_value(&self) -> (String, String) {
        (self.name().to_string(), self.value())
    }
}

impl fmt::Display for InterfaceOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.name(), self.value())
    }
}

/// Helper function to parse boolean values from interface files.
fn parse_bool(value: &str) -> bool {
    matches!(value.to_lowercase().as_str(), "yes" | "on" | "true" | "1")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_key_value_address() {
        let opt = InterfaceOption::from_key_value("address", "192.168.1.100");
        assert_eq!(opt, InterfaceOption::Address("192.168.1.100".to_string()));
        assert_eq!(opt.name(), "address");
        assert_eq!(opt.value(), "192.168.1.100");
    }

    #[test]
    fn test_from_key_value_mtu() {
        let opt = InterfaceOption::from_key_value("mtu", "9216");
        assert_eq!(opt, InterfaceOption::Mtu(9216));
        assert_eq!(opt.name(), "mtu");
        assert_eq!(opt.value(), "9216");
    }

    #[test]
    fn test_from_key_value_bridge_ports() {
        let opt = InterfaceOption::from_key_value("bridge-ports", "swp1 swp2 swp3");
        assert_eq!(
            opt,
            InterfaceOption::BridgePorts(vec![
                "swp1".to_string(),
                "swp2".to_string(),
                "swp3".to_string()
            ])
        );
        assert_eq!(opt.value(), "swp1 swp2 swp3");
    }

    #[test]
    fn test_from_key_value_bridge_vlan_aware() {
        let opt_yes = InterfaceOption::from_key_value("bridge-vlan-aware", "yes");
        assert_eq!(opt_yes, InterfaceOption::BridgeVlanAware(true));
        assert_eq!(opt_yes.value(), "yes");

        let opt_no = InterfaceOption::from_key_value("bridge-vlan-aware", "no");
        assert_eq!(opt_no, InterfaceOption::BridgeVlanAware(false));
        assert_eq!(opt_no.value(), "no");
    }

    #[test]
    fn test_from_key_value_unknown() {
        let opt = InterfaceOption::from_key_value("custom-option", "custom-value");
        assert_eq!(
            opt,
            InterfaceOption::Other("custom-option".to_string(), "custom-value".to_string())
        );
        assert_eq!(opt.name(), "custom-option");
        assert_eq!(opt.value(), "custom-value");
    }

    #[test]
    fn test_display() {
        let opt = InterfaceOption::Mtu(1500);
        assert_eq!(format!("{}", opt), "mtu 1500");

        let opt = InterfaceOption::BridgeVlanAware(true);
        assert_eq!(format!("{}", opt), "bridge-vlan-aware yes");
    }

    #[test]
    fn test_to_key_value() {
        let opt = InterfaceOption::Gateway("192.168.1.1".to_string());
        let (key, value) = opt.to_key_value();
        assert_eq!(key, "gateway");
        assert_eq!(value, "192.168.1.1");
    }
}

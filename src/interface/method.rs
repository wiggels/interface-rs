use std::fmt;
use std::str::FromStr;

/// Represents the configuration method in the `interfaces(5)` file.
///
/// The `Method` enum constrains the `method` field to known values, while still
/// allowing custom methods through the `Other` variant.
///
/// # Variants
///
/// - `Static`: Static IP configuration.
/// - `Dhcp`: DHCP client configuration.
/// - `Loopback`: Loopback interface configuration.
/// - `Manual`: Manual configuration (no automatic setup).
/// - `Other`: Any other method not explicitly supported.
///
/// # Examples
///
/// Parsing a `Method` from a string:
///
/// ```rust
/// use interface_rs::interface::Method;
/// use std::str::FromStr;
///
/// let method = Method::from_str("dhcp").unwrap();
/// assert_eq!(method, Method::Dhcp);
///
/// // Unknown methods are captured as Other
/// let custom = Method::from_str("ppp").unwrap();
/// assert_eq!(custom, Method::Other("ppp".to_string()));
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum Method {
    /// Static IP configuration.
    Static,
    /// DHCP client configuration.
    Dhcp,
    /// Loopback interface configuration.
    Loopback,
    /// Manual configuration (no automatic setup).
    Manual,
    /// Any other method not explicitly supported.
    Other(String),
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let method_str = match self {
            Method::Static => "static",
            Method::Dhcp => "dhcp",
            Method::Loopback => "loopback",
            Method::Manual => "manual",
            Method::Other(s) => s.as_str(),
        };
        write!(f, "{}", method_str)
    }
}

impl FromStr for Method {
    type Err = std::convert::Infallible;

    /// Parses a `Method` from a string slice.
    ///
    /// This implementation never fails - unknown methods are captured as `Other`.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "static" => Method::Static,
            "dhcp" => Method::Dhcp,
            "loopback" => Method::Loopback,
            "manual" => Method::Manual,
            other => Method::Other(other.to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_from_str() {
        assert_eq!(Method::from_str("static").unwrap(), Method::Static);
        assert_eq!(Method::from_str("dhcp").unwrap(), Method::Dhcp);
        assert_eq!(Method::from_str("loopback").unwrap(), Method::Loopback);
        assert_eq!(Method::from_str("manual").unwrap(), Method::Manual);
        assert_eq!(
            Method::from_str("ppp").unwrap(),
            Method::Other("ppp".to_string())
        );
    }

    #[test]
    fn test_method_display() {
        assert_eq!(Method::Static.to_string(), "static");
        assert_eq!(Method::Dhcp.to_string(), "dhcp");
        assert_eq!(Method::Loopback.to_string(), "loopback");
        assert_eq!(Method::Manual.to_string(), "manual");
        assert_eq!(Method::Other("ppp".to_string()).to_string(), "ppp");
    }
}

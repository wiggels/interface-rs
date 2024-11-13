use std::str::FromStr;
use std::fmt;
use std::error::Error;

/// Represents the address family in the `/etc/network/interfaces` file.
///
/// The `Family` enum constrains the `family` field to allowed values, ensuring
/// that only valid address families are used.
///
/// # Variants
///
/// - `Inet`: Represents the `inet` family (IPv4).
/// - `Inet6`: Represents the `inet6` family (IPv6).
/// - `IpX`: Represents the `ipx` family.
/// - `Can`: Represents the `can` family.
///
/// # Examples
///
/// Parsing a `Family` from a string:
///
/// ```rust
/// use interface_rs::interface::Family;
/// use std::str::FromStr;
///
/// let family = Family::from_str("inet").unwrap();
/// assert_eq!(family, Family::Inet);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum Family {
    /// The `inet` address family (IPv4).
    Inet,
    /// The `inet6` address family (IPv6).
    Inet6,
    /// The `ipx` address family.
    IpX,
    /// The `can` address family.
    Can,
}

impl fmt::Display for Family {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let family_str = match self {
            Family::Inet => "inet",
            Family::Inet6 => "inet6",
            Family::IpX => "ipx",
            Family::Can => "can",
        };
        write!(f, "{}", family_str)
    }
}

impl FromStr for Family {
    type Err = FamilyParseError;

    /// Parses a `Family` from a string slice.
    ///
    /// # Errors
    ///
    /// Returns a `FamilyParseError` if the input string does not match any known family.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "inet" => Ok(Family::Inet),
            "inet6" => Ok(Family::Inet6),
            "ipx" => Ok(Family::IpX),
            "can" => Ok(Family::Can),
            _ => Err(FamilyParseError(s.to_string())),
        }
    }
}

/// An error that occurs when parsing a `Family` from a string.
///
/// This error is returned when the input string does not correspond to any
/// known address family.
#[derive(Debug, Clone)]
pub struct FamilyParseError(pub String);

impl fmt::Display for FamilyParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid family: {}", self.0)
    }
}

impl Error for FamilyParseError {}

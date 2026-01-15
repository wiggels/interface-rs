use std::path::PathBuf;

/// Represents a `mapping` stanza in the `/etc/network/interfaces` file.
///
/// The `Mapping` struct holds the script and map entries associated with a
/// mapping configuration.
///
/// # Examples
///
/// Creating a new `Mapping`:
///
/// ```rust
/// use interface_rs::interface::Mapping;
/// use std::path::PathBuf;
///
/// let mapping = Mapping {
///     script: PathBuf::from("/usr/local/bin/map-scripts"),
///     maps: vec!["eth0".to_string(), "eth1".to_string()],
/// };
/// ```
#[derive(Debug, Clone)]
pub struct Mapping {
    /// The script to be used for mapping.
    pub script: PathBuf,
    /// A list of map entries.
    pub maps: Vec<String>,
}

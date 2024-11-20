use crate::error::NetworkInterfacesError;
use crate::interface::Interface;
use crate::parser::Parser;
use crate::helper::sort::natural;
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Represents the collection of network interfaces defined in an `interfaces(5)` file.
///
/// The `NetworkInterfaces` struct provides methods to load, manipulate, and save
/// network interface configurations.
///
/// # Examples
///
/// Loading and modifying interfaces:
///
/// ```rust
/// use interface_rs::NetworkInterfaces;
/// use interface_rs::interface::Interface;
///
/// let mut net_ifaces = NetworkInterfaces::load("tests/interfaces").unwrap();
///
/// // Modify an interface
/// if let Some(iface) = net_ifaces.get_interface_mut("eth0") {
///     iface.method = Some("static".to_string());
///     iface.options.push(("address".to_string(), "192.168.1.100".to_string()));
/// }
///
/// // Save changes
/// net_ifaces.save().unwrap();
/// ```
#[derive(Debug)]
pub struct NetworkInterfaces {
    /// A mapping of interface names to their configurations.
    interfaces: HashMap<String, Interface>,
    /// The path to the interfaces file.
    path: Option<PathBuf>,
    /// The last modified time of the interfaces file.
    last_modified: Option<SystemTime>,
    /// Comments from the original file
    comments: Vec<String>,
    /// Source directives from the original file
    sources: Vec<String>,
}

impl NetworkInterfaces {
    /// Creates a new `NetworkInterfaces` instance.
    fn new(
        interfaces: HashMap<String, Interface>,
        comments: Vec<String>,
        sources: Vec<String>,
        path: Option<PathBuf>,
        last_modified: Option<SystemTime>,
    ) -> Self {
        NetworkInterfaces {
            interfaces,
            comments,
            sources,
            path,
            last_modified,
        }
    }

    /// Loads the `interfaces(5)` file into memory.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the interfaces file.
    ///
    /// # Returns
    ///
    /// A `NetworkInterfaces` instance containing the parsed interfaces.
    ///
    /// # Errors
    ///
    /// Returns a `NetworkInterfacesError` if the file cannot be read or parsed.
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, NetworkInterfacesError> {
        let path_buf = path.as_ref().to_path_buf();
        let metadata = fs::metadata(&path_buf)?;
        let last_modified = metadata.modified()?;

        let content = fs::read_to_string(&path_buf)?;
        let parser = Parser::new();
        let (interfaces, comments, sources) = parser.parse(&content)?;

        Ok(NetworkInterfaces::new(
            interfaces,
            comments,
            sources,
            Some(path_buf),
            Some(last_modified),
        ))
    }

    /// Retrieves a reference to an interface by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the interface.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the `Interface` if found.
    pub fn get_interface(&self, name: &str) -> Option<&Interface> {
        self.interfaces.get(name)
    }

    /// Retrieves a mutable reference to an interface by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the interface.
    ///
    /// # Returns
    ///
    /// An `Option` containing a mutable reference to the `Interface` if found.
    pub fn get_interface_mut(&mut self, name: &str) -> Option<&mut Interface> {
        self.interfaces.get_mut(name)
    }

    /// Adds or updates an interface in the collection.
    ///
    /// # Arguments
    ///
    /// * `iface` - The `Interface` to add or update.
    pub fn add_interface(&mut self, iface: Interface) {
        self.interfaces.insert(iface.name.clone(), iface);
    }

    /// Deletes an interface by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the interface to delete.
    pub fn delete_interface(&mut self, name: &str) {
        self.interfaces.remove(name);
    }

    /// Returns the number of interfaces.
    pub fn len(&self) -> usize {
        self.interfaces.len()
    }

    /// Checks if the collection is empty.
    pub fn is_empty(&self) -> bool {
        self.interfaces.is_empty()
    }

    /// Finds the next unused VLAN ID within a specified range.
    ///
    /// # Arguments
    ///
    /// * `start` - The starting VLAN ID (inclusive).
    /// * `end` - The ending VLAN ID (inclusive).
    ///
    /// # Returns
    ///
    /// * `Option<u16>` - The next unused VLAN ID, or `None` if all are used.
    pub fn next_unused_vlan_in_range(&self, start: u16, end: u16) -> Option<u16> {
        for vlan_id in start..=end {
            let vlan_name = format!("vlan{}", vlan_id);
            if !self.interfaces.contains_key(&vlan_name) {
                return Some(vlan_id);
            }
        }
        None // All VLAN IDs in the specified range are used
    }

    /// Retrieves the VLAN associated with an existing VNI interface.
    ///
    /// # Arguments
    ///
    /// * `vni_id` - The VNI ID to search for (e.g., `1347682`).
    ///
    /// # Returns
    ///
    /// * `Option<u16>` - The VLAN ID specified in the `bridge-access` option, or `None` if
    ///   the interface does not exist or the option is not present.
    pub fn get_existing_vni_vlan(&self, vni_id: u32) -> Option<u16> {
        let vni_name = format!("vni{}", vni_id);

        // Check if the interface exists
        let interface = self.interfaces.get(&vni_name)?;

        // Look for the `bridge-access` option
        for (key, value) in &interface.options {
            if key == "bridge-access" {
                // Try to parse the value as a u16
                if let Ok(vlan_id) = value.parse::<u16>() {
                    return Some(vlan_id);
                }
            }
        }

        None // No `bridge-access` option or invalid value
    }

    /// Saves changes back to the `interfaces(5)` file.
    ///
    /// # Errors
    ///
    /// Returns a `NetworkInterfacesError` if the file cannot be written or has been modified on disk.
    pub fn save(&mut self) -> Result<(), NetworkInterfacesError> {
        let path = match &self.path {
            Some(p) => p.clone(),
            None => {
                return Err(NetworkInterfacesError::Other(
                    "No file path specified".to_string(),
                ))
            }
        };

        // Check if file has been modified since last load
        let metadata = fs::metadata(&path)?;
        let current_modified = metadata.modified()?;
        if let Some(last_modified) = self.last_modified {
            if current_modified > last_modified {
                // File has been modified since last read
                return Err(NetworkInterfacesError::FileModified);
            }
        }

        // Write to the file using Display implementation
        let mut file = fs::File::create(&path)?;
        write!(file, "{}", self)?;

        // Update last_modified
        self.last_modified = Some(SystemTime::now());
        Ok(())
    }

    /// Reloads the interfaces file from disk.
    ///
    /// # Errors
    ///
    /// Returns a `NetworkInterfacesError` if the file cannot be read or parsed.
    pub fn reload(&mut self) -> Result<(), NetworkInterfacesError> {
        let path = match &self.path {
            Some(p) => p.clone(),
            None => {
                return Err(NetworkInterfacesError::Other(
                    "No file path specified".to_string(),
                ))
            }
        };
        let reloaded = NetworkInterfaces::load(path)?;
        self.interfaces = reloaded.interfaces;
        self.comments = reloaded.comments;
        self.sources = reloaded.sources;
        self.last_modified = reloaded.last_modified;
        Ok(())
    }
}

// Implement Display for NetworkInterfaces to allow easy printing
impl fmt::Display for NetworkInterfaces {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Print comments at the top if any
        for comment in &self.comments {
            writeln!(f, "{}", comment)?;
        }

        // Print source directives if any
        for source in &self.sources {
            writeln!(f, "{}", source)?;
        }

        // Collect interfaces into a vector and sort them by name
        let mut interfaces: Vec<&Interface> = self.interfaces.values().collect();
        interfaces.sort_by(|a, b| natural(&a.name, &b.name));

        // Print interfaces
        for iface in interfaces {
            writeln!(f)?;
            write!(f, "{}", iface)?;
        }
        Ok(())
    }
}

// Implement methods to access interfaces directly if needed
impl NetworkInterfaces {
    /// Returns an iterator over the interfaces.
    pub fn iter(&self) -> impl Iterator<Item = (&String, &Interface)> {
        self.interfaces.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_unused_vlan_in_range() {
        // Create a `NetworkInterfaces` instance with some used VLANs
        let mut network_interfaces = NetworkInterfaces {
            interfaces: HashMap::new(),
            path: None,
            last_modified: None,
            comments: Vec::new(),
            sources: Vec::new(),
        };

        // Add some VLAN interfaces to simulate used IDs
        network_interfaces.add_interface(Interface::builder("vlan1000").build());
        network_interfaces.add_interface(Interface::builder("vlan1001").build());
        network_interfaces.add_interface(Interface::builder("vlan1003").build());

        // Test: Find the next unused VLAN ID in the range 1000 to 1999
        let next_vlan_id = network_interfaces.next_unused_vlan_in_range(1000, 1999);
        assert_eq!(next_vlan_id, Some(1002));

        // Test: Find the next unused VLAN ID when all VLANs are used in the range
        network_interfaces.add_interface(Interface::builder("vlan1002").build());
        let next_vlan_id = network_interfaces.next_unused_vlan_in_range(1000, 1003);
        assert_eq!(next_vlan_id, None);

        // Test: Find the next unused VLAN ID in a different range
        let next_vlan_id = network_interfaces.next_unused_vlan_in_range(2000, 2005);
        assert_eq!(next_vlan_id, Some(2000));
    }

    #[test]
    fn test_get_existing_vni_vlan() {
        let mut network_interfaces = NetworkInterfaces {
            interfaces: HashMap::new(),
            path: None,
            last_modified: None,
            comments: Vec::new(),
            sources: Vec::new(),
        };

        // Add a VNI interface
        network_interfaces.add_interface(
            Interface::builder("vni123456")
                .with_auto(true)
                .with_option("bridge-access", "1002")
                .build(),
        );

        // Test: Find VLAN ID for existing VNI
        assert_eq!(network_interfaces.get_existing_vni_vlan(123456), Some(1002));

        // Test: No `bridge-access` option
        network_interfaces.add_interface(Interface::builder("vni987654").with_auto(true).build());
        assert_eq!(network_interfaces.get_existing_vni_vlan(987654), None);

        // Test: Nonexistent VNI
        assert_eq!(network_interfaces.get_existing_vni_vlan(666), None);
    }
}

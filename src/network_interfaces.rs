use crate::interface::Interface;
use crate::parser::Parser;
use crate::error::NetworkInterfacesError;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use std::fmt;

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
/// let mut net_ifaces = NetworkInterfaces::load("/path/to/interfaces").unwrap();
///
/// // Modify an interface
/// if let Some(iface) = net_ifaces.interfaces.get_mut("eth0") {
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
    pub interfaces: HashMap<String, Interface>,
    /// The path to the interfaces file.
    path: Option<PathBuf>,
    /// The last modified time of the interfaces file.
    last_modified: Option<SystemTime>,
}

impl NetworkInterfaces {
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
        let interfaces = parser.parse(&content)?;

        Ok(NetworkInterfaces {
            interfaces,
            path: Some(path_buf),
            last_modified: Some(last_modified),
        })
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

        let mut file = fs::File::create(&path)?;

        // Write interfaces to file
        for iface in self.interfaces.values() {
            write!(file, "{}", iface)?;
            writeln!(file)?; // Blank line between interfaces
        }

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
        let reloaded = NetworkInterfaces::load(&path)?;
        self.interfaces = reloaded.interfaces;
        self.last_modified = reloaded.last_modified;
        Ok(())
    }
}

// Implement Display for NetworkInterfaces to allow easy printing
impl fmt::Display for NetworkInterfaces {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for iface in self.interfaces.values() {
            write!(f, "{}\n", iface)?;
        }
        Ok(())
    }
}

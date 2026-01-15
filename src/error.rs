use crate::interface::FamilyParseError;
use std::error::Error;
use std::fmt;
use std::io;

/// The main error type for the `NetworkInterfaces` library.
///
/// This enum encapsulates all possible errors that can occur within the library.
#[derive(Debug)]
pub enum NetworkInterfacesError {
    /// An I/O error occurred.
    Io(io::Error),
    /// An error occurred while parsing the interfaces file.
    Parser(ParserError),
    /// An error occurred while parsing the `Family` enum.
    FamilyParse(FamilyParseError),
    /// The interfaces file has been modified on disk since it was last loaded.
    FileModified,
    /// A catch-all for other errors.
    Other(String),
}

impl fmt::Display for NetworkInterfacesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NetworkInterfacesError::Io(err) => write!(f, "I/O error: {}", err),
            NetworkInterfacesError::Parser(err) => write!(f, "Parser error: {}", err),
            NetworkInterfacesError::FamilyParse(err) => write!(f, "Family parse error: {}", err),
            NetworkInterfacesError::FileModified => write!(
                f,
                "The interfaces file has been modified on disk since it was last loaded."
            ),
            NetworkInterfacesError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl Error for NetworkInterfacesError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            NetworkInterfacesError::Io(err) => Some(err),
            NetworkInterfacesError::Parser(err) => Some(err),
            NetworkInterfacesError::FamilyParse(err) => Some(err),
            NetworkInterfacesError::FileModified => None,
            NetworkInterfacesError::Other(_) => None,
        }
    }
}

impl From<io::Error> for NetworkInterfacesError {
    fn from(err: io::Error) -> Self {
        NetworkInterfacesError::Io(err)
    }
}

impl From<ParserError> for NetworkInterfacesError {
    fn from(err: ParserError) -> Self {
        NetworkInterfacesError::Parser(err)
    }
}

impl From<FamilyParseError> for NetworkInterfacesError {
    fn from(err: FamilyParseError) -> Self {
        NetworkInterfacesError::FamilyParse(err)
    }
}

/// Represents errors that can occur during parsing of the interfaces file.
#[derive(Debug, Clone)]
pub struct ParserError {
    /// A message describing the parsing error.
    pub message: String,
    /// Optional line number where the error occurred.
    pub line: Option<usize>,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(line) = self.line {
            write!(f, "Parser error on line {}: {}", line, self.message)
        } else {
            write!(f, "Parser error: {}", self.message)
        }
    }
}

impl Error for ParserError {}

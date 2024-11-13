use crate::interface::{Family, Interface, Mapping};
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
        let mut interfaces: HashMap<String, Interface> = HashMap::new();
        let mut lines_iter = content.lines().enumerate().peekable();

        while let Some((line_number, line)) = lines_iter.next() {
            let line_number = line_number + 1; // Make line numbers 1-based
            let line = line.trim_end();

            if line.trim().is_empty() || line.trim_start().starts_with('#') {
                continue;
            }

            // Handle line continuation
            let mut current_line = line.to_string();
            while current_line.trim_end().ends_with('\\') {
                current_line =
                    current_line.trim_end_matches('\\').to_string();
                if let Some((_, next_line)) = lines_iter.next() {
                    current_line.push_str(next_line.trim_start());
                } else {
                    break;
                }
            }

            let tokens: Vec<&str> = current_line
                .trim()
                .split_whitespace()
                .collect();

            if tokens.is_empty() {
                continue;
            }

            match tokens[0] {
                "auto" => {
                    for iface_name in &tokens[1..] {
                        let iface = interfaces
                            .entry(iface_name.to_string())
                            .or_insert_with(|| Interface {
                                name: iface_name.to_string(),
                                auto: true,
                                allow: Vec::new(),
                                family: None,
                                method: None,
                                options: Vec::new(),
                                mapping: None,
                            });
                        iface.auto = true;
                    }
                }
                s if s.starts_with("allow-") => {
                    let allow_type =
                        s.trim_start_matches("allow-").to_string();
                    for iface_name in &tokens[1..] {
                        let iface = interfaces
                            .entry(iface_name.to_string())
                            .or_insert_with(|| Interface {
                                name: iface_name.to_string(),
                                auto: false,
                                allow: vec![allow_type.clone()],
                                family: None,
                                method: None,
                                options: Vec::new(),
                                mapping: None,
                            });
                        iface.allow.push(allow_type.clone());
                    }
                }
                "iface" => {
                    let name = tokens.get(1).ok_or_else(|| ParserError {
                        message: "Missing interface name in 'iface' stanza"
                            .to_string(),
                        line: Some(line_number),
                    })?.to_string();

                    let family_str = tokens.get(2).ok_or_else(|| ParserError {
                        message: "Missing family in 'iface' stanza"
                            .to_string(),
                        line: Some(line_number),
                    })?;

                    let method_str = tokens.get(3).ok_or_else(|| ParserError {
                        message: "Missing method in 'iface' stanza"
                            .to_string(),
                        line: Some(line_number),
                    })?.to_string();

                    let family = family_str.parse::<Family>()
                        .map_err(|e| ParserError {
                            message: e.to_string(),
                            line: Some(line_number),
                        })?;

                    let iface = interfaces.entry(name.clone())
                        .or_insert_with(|| Interface {
                            name: name.clone(),
                            auto: false,
                            allow: Vec::new(),
                            family: Some(family.clone()),
                            method: Some(method_str.clone()),
                            options: Vec::new(),
                            mapping: None,
                        });
                    iface.family = Some(family);
                    iface.method = Some(method_str);

                    // Parse options
                    while let Some(&(next_line_number, next_line)) =
                        lines_iter.peek()
                    {
                        let next_line_number = next_line_number + 1;
                        let next_line = next_line.trim_end();
                        if next_line.trim().is_empty()
                            || next_line.trim_start().starts_with('#')
                        {
                            lines_iter.next();
                            continue;
                        }
                        if next_line.starts_with(char::is_whitespace) {
                            let (_, option_line) = lines_iter.next()
                                .unwrap();
                            let option_line = option_line.trim();
                            let mut option_tokens = option_line
                                .split_whitespace();
                            if let Some(option_name) =
                                option_tokens.next()
                            {
                                let option_value = option_tokens
                                    .collect::<Vec<&str>>()
                                    .join(" ");
                                iface.options.push((
                                    option_name.to_string(),
                                    option_value,
                                ));
                            } else {
                                return Err(ParserError {
                                    message: "Invalid option syntax"
                                        .to_string(),
                                    line: Some(next_line_number),
                                });
                            }
                        } else {
                            break;
                        }
                    }
                }
                "mapping" => {
                    let interface_pattern =
                        tokens.get(1).ok_or_else(|| ParserError {
                            message:
                                "Missing interface pattern in 'mapping' stanza"
                                    .to_string(),
                            line: Some(line_number),
                        })?.to_string();

                    let mut script = None;
                    let mut maps = Vec::new();

                    while let Some(&(next_line_number, next_line)) =
                        lines_iter.peek()
                    {
                        let next_line_number = next_line_number + 1;
                        let next_line = next_line.trim_end();
                        if next_line.trim().is_empty()
                            || next_line.trim_start().starts_with('#')
                        {
                            lines_iter.next();
                            continue;
                        }
                        if next_line.starts_with(char::is_whitespace) {
                            let (_, indented_line) = lines_iter.next()
                                .unwrap();
                            let indented_line = indented_line.trim();
                            let mut indented_tokens = indented_line
                                .split_whitespace();
                            if let Some(first_token) =
                                indented_tokens.next()
                            {
                                match first_token {
                                    "script" => {
                                        script = Some(indented_tokens
                                            .collect::<Vec<&str>>()
                                            .join(" "));
                                    }
                                    "map" => {
                                        let map_value = indented_tokens
                                            .collect::<Vec<&str>>()
                                            .join(" ");
                                        maps.push(map_value);
                                    }
                                    _ => {
                                        return Err(ParserError {
                                            message: format!(
                                                "Unknown token '{}' in 'mapping' stanza",
                                                first_token
                                            ),
                                            line: Some(next_line_number),
                                        });
                                    }
                                }
                            } else {
                                return Err(ParserError {
                                    message:
                                        "Invalid syntax in 'mapping' stanza"
                                            .to_string(),
                                    line: Some(next_line_number),
                                });
                            }
                        } else {
                            break;
                        }
                    }

                    let script = script.ok_or_else(|| ParserError {
                        message:
                            "Missing 'script' in 'mapping' stanza"
                                .to_string(),
                        line: Some(line_number),
                    })?;

                    let iface = interfaces.entry(interface_pattern.clone())
                        .or_insert_with(|| Interface {
                            name: interface_pattern.clone(),
                            auto: false,
                            allow: Vec::new(),
                            family: None,
                            method: None,
                            options: Vec::new(),
                            mapping: None,
                        });
                    iface.mapping = Some(Mapping { script, maps });
                }
                _ => {
                    // Handle other stanzas or ignore
                    // For now, we ignore unknown stanzas
                }
            }
        }

        Ok(interfaces)
    }
}

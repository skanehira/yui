use thiserror::Error;

/// Context information about an object file in linking process
#[derive(Debug, Clone, PartialEq)]
pub struct ObjectContext {
    pub file_name: String,
    pub object_index: usize,
}

/// Information about an unresolved symbol
#[derive(Debug, Clone, PartialEq)]
pub struct UnresolvedSymbol {
    pub name: String,
    pub referenced_from: Vec<ObjectContext>,
}

/// Unified error type for the Yui linker
#[derive(Error, Debug)]
pub enum LinkerError {
    /// Symbol is defined in multiple object files
    #[error("Duplicate symbol '{symbol_name}' found: first defined in {first_file} (object {first_idx}), redefined in {dup_file} (object {dup_idx})", 
        symbol_name = symbol_name,
        first_file = first_definition.file_name,
        first_idx = first_definition.object_index,
        dup_file = duplicate_definition.file_name,
        dup_idx = duplicate_definition.object_index
    )]
    DuplicateSymbol {
        symbol_name: String,
        first_definition: ObjectContext,
        duplicate_definition: ObjectContext,
    },

    /// One or more symbols could not be resolved
    #[error("Unresolved symbols found:\n{}", format_unresolved_symbols(.symbols))]
    UnresolvedSymbols { symbols: Vec<UnresolvedSymbol> },

    /// Entry point symbol not found
    #[error("Entry point symbol '{entry_symbol}' not found")]
    MissingEntryPoint { entry_symbol: String },

    /// Required section not found
    #[error("Section '{section_name}' not found{}", context.as_ref().map(|c| format!(" ({})", c)).unwrap_or_default())]
    SectionNotFound {
        section_name: String,
        context: Option<String>,
    },

    /// Error during relocation processing
    #[error("Relocation error: {message}{}{}{}", 
        symbol_name.as_ref().map(|s| format!(" (symbol: {})", s)).unwrap_or_default(),
        object_context.as_ref().map(|o| format!(" (object: {})", o.file_name)).unwrap_or_default(),
        relocation_type.as_ref().map(|t| format!(" (type: {})", t)).unwrap_or_default()
    )]
    RelocationError {
        message: String,
        symbol_name: Option<String>,
        object_context: Option<ObjectContext>,
        relocation_type: Option<String>,
    },

    /// Error from the ELF parser
    #[error("Parse error{}: {error}", context.as_ref().map(|c| format!(" ({})", c)).unwrap_or_default())]
    Parse {
        #[source]
        error: crate::parser::error::ParseError,
        context: Option<String>,
    },

    /// I/O error
    #[error("I/O error{}: {error}", context.as_ref().map(|c| format!(" ({})", c)).unwrap_or_default())]
    Io {
        #[source]
        error: std::io::Error,
        context: Option<String>,
    },

    /// Generic error for other cases
    #[error("Linker error{}: {message}", context.as_ref().map(|c| format!(" ({})", c)).unwrap_or_default())]
    Generic {
        message: String,
        context: Option<String>,
    },
}

fn format_unresolved_symbols(symbols: &[UnresolvedSymbol]) -> String {
    symbols
        .iter()
        .map(|s| {
            let refs = s
                .referenced_from
                .iter()
                .map(|obj| obj.file_name.as_str())
                .collect::<Vec<_>>()
                .join(", ");
            format!("- {} (referenced from {})", s.name, refs)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

impl From<std::io::Error> for LinkerError {
    fn from(error: std::io::Error) -> Self {
        LinkerError::Io {
            error,
            context: None,
        }
    }
}

impl From<crate::parser::error::ParseError> for LinkerError {
    fn from(error: crate::parser::error::ParseError) -> Self {
        LinkerError::Parse {
            error,
            context: None,
        }
    }
}

impl LinkerError {
    /// Add context to an error
    pub fn with_context<S: Into<String>>(mut self, context: S) -> Self {
        match &mut self {
            LinkerError::Parse { context: c, .. }
            | LinkerError::Io { context: c, .. }
            | LinkerError::SectionNotFound { context: c, .. }
            | LinkerError::Generic { context: c, .. } => {
                *c = Some(context.into());
            }
            _ => {}
        }
        self
    }

    /// Create a new relocation error with full context
    pub fn relocation_error<S: Into<String>>(
        message: S,
        symbol_name: Option<String>,
        object_context: Option<ObjectContext>,
        relocation_type: Option<String>,
    ) -> Self {
        LinkerError::RelocationError {
            message: message.into(),
            symbol_name,
            object_context,
            relocation_type,
        }
    }

    /// Create a duplicate symbol error
    pub fn duplicate_symbol<S: Into<String>>(
        symbol_name: S,
        first_definition: ObjectContext,
        duplicate_definition: ObjectContext,
    ) -> Self {
        LinkerError::DuplicateSymbol {
            symbol_name: symbol_name.into(),
            first_definition,
            duplicate_definition,
        }
    }

    /// Create an unresolved symbols error
    pub fn unresolved_symbols(symbols: Vec<UnresolvedSymbol>) -> Self {
        LinkerError::UnresolvedSymbols { symbols }
    }
}

pub type Result<T> = std::result::Result<T, LinkerError>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error as StdError;

    #[test]
    fn test_duplicate_symbol_error_display() {
        let error = LinkerError::DuplicateSymbol {
            symbol_name: "main".to_string(),
            first_definition: ObjectContext {
                file_name: "main.o".to_string(),
                object_index: 0,
            },
            duplicate_definition: ObjectContext {
                file_name: "duplicate.o".to_string(),
                object_index: 1,
            },
        };

        let expected = "Duplicate symbol 'main' found: first defined in main.o (object 0), redefined in duplicate.o (object 1)";
        assert_eq!(error.to_string(), expected);
    }

    #[test]
    fn test_unresolved_symbols_error_display() {
        let error = LinkerError::UnresolvedSymbols {
            symbols: vec![
                UnresolvedSymbol {
                    name: "printf".to_string(),
                    referenced_from: vec![ObjectContext {
                        file_name: "main.o".to_string(),
                        object_index: 0,
                    }],
                },
                UnresolvedSymbol {
                    name: "malloc".to_string(),
                    referenced_from: vec![ObjectContext {
                        file_name: "util.o".to_string(),
                        object_index: 1,
                    }],
                },
            ],
        };

        assert_eq!(
            error.to_string(),
            "Unresolved symbols found:\n- printf (referenced from main.o)\n- malloc (referenced from util.o)"
        );
    }

    #[test]
    fn test_parse_error_wrapping() {
        let parse_error = crate::parser::error::ParseError::InvalidClass(255);
        let linker_error = LinkerError::Parse {
            error: parse_error,
            context: Some("parsing main.o".to_string()),
        };

        assert_eq!(
            linker_error.to_string(),
            "Parse error (parsing main.o): Invalid ELF class: 255"
        );
    }

    #[test]
    fn test_relocation_error_display() {
        let error = LinkerError::RelocationError {
            message: "Symbol index out of range".to_string(),
            symbol_name: Some("main".to_string()),
            object_context: Some(ObjectContext {
                file_name: "main.o".to_string(),
                object_index: 0,
            }),
            relocation_type: Some("R_AARCH64_ADR_PREL_LO21".to_string()),
        };

        assert_eq!(
            error.to_string(),
            "Relocation error: Symbol index out of range (symbol: main) (object: main.o) (type: R_AARCH64_ADR_PREL_LO21)"
        );
    }

    #[test]
    fn test_io_error_wrapping() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let linker_error = LinkerError::Io {
            error: io_error,
            context: Some("reading input file".to_string()),
        };

        assert_eq!(
            linker_error.to_string(),
            "I/O error (reading input file): File not found"
        );
    }

    #[test]
    fn test_error_trait_implementation() {
        let error = LinkerError::DuplicateSymbol {
            symbol_name: "test".to_string(),
            first_definition: ObjectContext {
                file_name: "first.o".to_string(),
                object_index: 0,
            },
            duplicate_definition: ObjectContext {
                file_name: "second.o".to_string(),
                object_index: 1,
            },
        };

        // Test that it implements std::error::Error
        let _: &dyn StdError = &error;

        // Test source() method
        assert!(error.source().is_none());
    }

    #[test]
    fn test_missing_entry_point_error() {
        let error = LinkerError::MissingEntryPoint {
            entry_symbol: "_start".to_string(),
        };

        assert_eq!(error.to_string(), "Entry point symbol '_start' not found");
    }

    #[test]
    fn test_section_not_found_error() {
        let error = LinkerError::SectionNotFound {
            section_name: ".text".to_string(),
            context: Some("applying relocations".to_string()),
        };

        assert_eq!(
            error.to_string(),
            "Section '.text' not found (applying relocations)"
        );
    }

    /// Test that demonstrates improved error reporting with structured information
    #[test]
    fn test_error_structured_information() {
        // Create an error with context
        let context = ObjectContext {
            file_name: "test.o".to_string(),
            object_index: 0,
        };

        let error = LinkerError::relocation_error(
            "Invalid relocation offset",
            Some("main".to_string()),
            Some(context),
            Some("R_AARCH64_ADR_PREL_LO21".to_string()),
        );

        assert_eq!(
            error.to_string(),
            "Relocation error: Invalid relocation offset (symbol: main) (object: test.o) (type: R_AARCH64_ADR_PREL_LO21)"
        );
    }

    /// Test that demonstrates multiple reference tracking for unresolved symbols
    #[test]
    fn test_unresolved_symbol_multiple_references() {
        let error = LinkerError::unresolved_symbols(vec![UnresolvedSymbol {
            name: "printf".to_string(),
            referenced_from: vec![
                ObjectContext {
                    file_name: "main.o".to_string(),
                    object_index: 0,
                },
                ObjectContext {
                    file_name: "utils.o".to_string(),
                    object_index: 1,
                },
            ],
        }]);

        assert_eq!(
            error.to_string(),
            "Unresolved symbols found:\n- printf (referenced from main.o, utils.o)"
        );
    }
}

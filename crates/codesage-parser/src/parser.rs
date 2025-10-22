//! Code parser implementation

use codesage_core::{CodeSageError, Language, Result};
use std::path::Path;
use tree_sitter::{Parser, Tree};

/// Main code parser
pub struct CodeParser {
    parser: Parser,
}

impl CodeParser {
    /// Create a new parser
    pub fn new() -> Self {
        Self {
            parser: Parser::new(),
        }
    }

    /// Parse a file
    pub fn parse_file(&self, path: &Path) -> Result<ParsedCode> {
        let extension = path
            .extension()
            .and_then(|e| e.to_str())
            .ok_or_else(|| CodeSageError::ParseError("No file extension".to_string()))?;

        let language = Language::from_extension(extension)
            .ok_or_else(|| CodeSageError::UnsupportedLanguage(extension.to_string()))?;

        let source = std::fs::read_to_string(path)
            .map_err(|e| CodeSageError::IoError(e))?;

        self.parse_source(&source, language)
    }

    /// Parse source code string
    pub fn parse_source(&self, source: &str, language: Language) -> Result<ParsedCode> {
        // Note: Full tree-sitter language support requires language-specific parsers
        // For now, we store the source and provide basic structure

        Ok(ParsedCode {
            language,
            source: source.to_string(),
            tree: None, // Will be populated when language parser is set
        })
    }

    /// Set the language for the parser
    pub fn set_language(&mut self, language: Language) -> Result<()> {
        // This would be implemented with actual tree-sitter language parsers
        // For now, we'll leave it as a placeholder
        match language {
            Language::Rust => {
                // parser.set_language(&tree_sitter_rust::LANGUAGE.into())?;
                Err(CodeSageError::ParseError(
                    "Rust parser not yet initialized".to_string(),
                ))
            }
            _ => Err(CodeSageError::UnsupportedLanguage(format!(
                "{:?}",
                language
            ))),
        }
    }
}

impl Default for CodeParser {
    fn default() -> Self {
        Self::new()
    }
}

/// Parsed code representation
#[derive(Debug)]
pub struct ParsedCode {
    pub language: Language,
    pub source: String,
    pub tree: Option<Tree>,
}

impl ParsedCode {
    /// Get the number of lines in the source
    pub fn line_count(&self) -> usize {
        self.source.lines().count()
    }

    /// Check if the code is empty
    pub fn is_empty(&self) -> bool {
        self.source.trim().is_empty()
    }

    /// Get source code as string
    pub fn source(&self) -> &str {
        &self.source
    }
}

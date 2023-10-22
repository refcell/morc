use serde::{Deserialize, Serialize};

/// A markdown level (1-6) is the level of a header.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MarkdownLevel(u8);

impl MarkdownLevel {
    /// Create a new markdown level.
    pub fn new(level: u8) -> Self {
        Self(level)
    }
}

/// Validate
///
/// A trait for validating markdown content.
pub trait Validate {
    /// Validate the markdown content.
    fn validate(&self) -> bool;
}

/// The Markdown Document.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Document {
    /// The top-level sections of the document.
    pub sections: Vec<Box<Section>>,
}

impl Document {
    /// Create a new document.
    pub fn new(sections: Vec<Box<Section>>) -> Self {
        Self { sections }
    }
}

impl Validate for Document {
    fn validate(&self) -> bool {
        self.sections.iter().all(|s| s.validate())
    }
}

/// A Markdown Section
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Section {
    /// The section [Header].
    pub header: Option<Header>,
    /// Body of the section.
    pub body: Option<String>,
    /// Subsections inside the section.
    pub subsections: Vec<Box<Section>>,
}

impl Section {
    /// Checks if the provided [Option<Header>] is a valid subsection of this section.
    pub fn is_subsection(&self, h: &Option<Header>) -> bool {
        self.header
            .as_ref()
            .map(|s| s.is_subsection(h))
            .unwrap_or(false)
    }
}

impl Validate for Section {
    fn validate(&self) -> bool {
        self.subsections
            .iter()
            .all(|s| s.validate() && self.is_subsection(&s.header))
    }
}

/// A Markdown Header.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Header {
    /// The level of the header.
    pub level: MarkdownLevel,
    /// The content of the header.
    pub content: String,
}

impl Header {
    /// Create a new header.
    pub fn new(level: u8, content: String) -> Self {
        Self {
            level: MarkdownLevel(level),
            content,
        }
    }

    /// Returns the inner header level as a u8.
    pub fn level(&self) -> u8 {
        self.level.0
    }

    /// Compares the header level to the given potential level.
    pub fn is_subsection(&self, h: &Option<Header>) -> bool {
        h.as_ref()
            .map(|h| h.level() > self.level())
            .unwrap_or(false)
    }
}

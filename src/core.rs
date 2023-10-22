use serde::{Deserialize, Serialize};

/// A Markdown Header.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Header {
    /// The level of the header.
    pub level: u8,
    /// The content of the header.
    pub content: String,
}

impl Header {
    /// Create a new header.
    pub fn new(level: u8, content: String) -> Self {
        Self { level, content }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_header() {
        let header = Header {
            level: 1,
            content: "Hello".to_string(),
        };
        assert_eq!(
            serde_json::to_string(&header).unwrap(),
            r#"{"level":1,"content":"Hello"}"#
        );
    }
}

use morc::core::{Document, Header, Section, Validate};

#[test]
fn test_validate_simple_document() {
    let doc = Document {
        sections: vec![Box::new(Section {
            header: Some(Header::new(1, "Hello".to_string())),
            body: None,
            subsections: vec![],
        })],
    };
    assert!(doc.validate());
}

// Helper function that recursively creates subsections
// with monotonically increasing levels until the desired
// depth is reached.
fn create_sections(depth: u8, desired: u8) -> Vec<Box<Section>> {
    if depth >= desired {
        return vec![];
    }
    let section = Box::new(Section {
        header: Some(Header::new(depth, "Hello".to_string())),
        body: None,
        subsections: create_sections(depth + 1, desired),
    });
    // Create a random number of sections at this level between 1 and 3.
    let mut sections = vec![section];
    let num_sections = rand::random::<u8>() % 3 + 1;
    for _ in 0..num_sections {
        sections.push(Box::new(Section {
            header: Some(Header::new(depth, "Hello".to_string())),
            body: None,
            subsections: create_sections(depth + 1, desired),
        }));
    }
    sections
}

#[test]
fn test_validate_n_nested_document() {
    let doc = Document::new(create_sections(1, 4));
    assert!(doc.validate());
}

#[test]
fn test_header() {
    let header = Header::new(1, "Hello".to_string());
    assert_eq!(
        serde_json::to_string(&header).unwrap(),
        r#"{"level":1,"content":"Hello"}"#
    );
}

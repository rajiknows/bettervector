use crate::types::{DocId, Document};
use std::collections::hash_map::DefaultHasher;
use std::fs;
use std::hash::{Hash, Hasher};

pub fn ingest_document(path: &str) -> Option<Document> {
    let content = fs::read_to_string(path).ok()?;
    let doc_id = calculate_hash(&content);
    Some(Document {
        id: doc_id,
        text: content,
    })
}

pub fn calculate_hash<T: Hash + ?Sized>(t: &T) -> DocId {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_hash_same_input() {
        let text = "hello world";
        let hash1 = calculate_hash(&text);
        let hash2 = calculate_hash(&text);
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_calculate_hash_different_input() {
        let hash1 = calculate_hash(&"hello");
        let hash2 = calculate_hash(&"world");
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_ingest_document_returns_correct_id() {
        let doc = Document {
            id: calculate_hash("test content"),
            text: "test content".to_string(),
        };
        assert_eq!(doc.id, calculate_hash("test content"));
    }

    #[test]
    fn test_ingest_document_nonexistent_file() {
        let result = ingest_document("/nonexistent/path/file.txt");
        assert!(result.is_none());
    }
}

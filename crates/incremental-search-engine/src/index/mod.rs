use crate::{
    query::tokenize,
    types::{DocId, Document},
};
use std::collections::HashMap;

pub struct InvertedIndex {
    // term -> list of (doc_id, term_freq)
    pub postings: HashMap<String, Vec<(DocId, u32)>>,
    // the df is can be derived from the postings , right ??
    // for example :
    //      let n_docs = postings.get(term).len();
    pub total_docs: usize,
}

impl InvertedIndex {
    pub fn new() -> Self {
        Self {
            postings: HashMap::new(),
            total_docs: 0,
        }
    }

    pub fn add_document(&mut self, doc: Document) -> bool {
        if self.contains_doc(doc.id) {
            return false;
        }

        let tokenized_data = tokenize(doc.text);
        let mut term_counts: HashMap<String, u32> = HashMap::new();
        for term in tokenized_data {
            *term_counts.entry(term).or_insert(0) += 1;
        }

        for (term, count) in term_counts {
            let postings_list = self.postings.entry(term).or_insert_with(Vec::new);
            postings_list.push((doc.id, count));
        }

        self.total_docs += 1;
        true
    }

    pub fn contains_doc(&self, doc_id: DocId) -> bool {
        self.postings
            .values()
            .any(|postings| postings.iter().any(|(id, _)| *id == doc_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_doc(id: DocId, text: &str) -> Document {
        Document {
            id,
            text: text.to_string(),
        }
    }

    #[test]
    fn test_new_index_is_empty() {
        let index = InvertedIndex::new();
        assert!(index.postings.is_empty());
    }

    #[test]
    fn test_add_document_creates_postings() {
        let mut index = InvertedIndex::new();
        let doc = make_doc(1, "hello world");

        index.add_document(doc);

        assert!(index.postings.contains_key("hello"));
        assert!(index.postings.contains_key("world"));
    }

    #[test]
    fn test_add_document_returns_true_on_success() {
        let mut index = InvertedIndex::new();
        let doc = make_doc(1, "hello");
        assert!(index.add_document(doc));
    }

    #[test]
    fn test_add_duplicate_document_returns_false() {
        let mut index = InvertedIndex::new();
        let doc1 = make_doc(1, "hello");
        let doc2 = make_doc(1, "world"); // same ID, different content

        index.add_document(doc1);
        assert!(!index.add_document(doc2));
    }

    #[test]
    fn test_contains_doc_returns_true_for_existing() {
        let mut index = InvertedIndex::new();
        let doc = make_doc(42, "hello");
        index.add_document(doc);

        assert!(index.contains_doc(42));
    }

    #[test]
    fn test_contains_doc_returns_false_for_nonexistent() {
        let index = InvertedIndex::new();
        assert!(!index.contains_doc(999));
    }

    #[test]
    fn test_multiple_docs_same_term() {
        let mut index = InvertedIndex::new();
        index.add_document(make_doc(1, "hello"));
        index.add_document(make_doc(2, "hello"));
        index.add_document(make_doc(3, "hello"));

        let postings = index.postings.get("hello").unwrap();
        assert_eq!(postings.len(), 3);
    }
}

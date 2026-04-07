use std::collections::HashMap;
use crate::types::{Document, DocId};

pub struct InvertedIndex {
    // term -> list of (doc_id, term_freq)
    pub postings: HashMap<String, Vec<(DocId, u32)>>,
}

impl InvertedIndex {
    pub fn new() -> Self {
        Self {
            postings: HashMap::new(),
        }
    }

    pub fn add_document(&mut self, doc: Document) {
        // tokenize → update postings
    }
}
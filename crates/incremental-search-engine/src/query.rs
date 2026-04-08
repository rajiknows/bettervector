use std::collections::HashMap;

use crate::{
    index::InvertedIndex,
    types::{DocId, Document},
};

pub fn search(index: &InvertedIndex, query: &str) -> Option<Vec<DocId>> {
    // tokenize the query
    let tokenized_query = tokenize(query.into());

    // doc_ids -> term_count
    let mut doc_ids_to_term: HashMap<DocId, usize> = HashMap::new();

    for term in tokenized_query {
        // search for this term in the postings
        if let Some(matched) = index.postings.get(&term) {
            // so these documents have this word
            // the document having the majority of query terms is ranked top
            for doc_tf in matched {
                *doc_ids_to_term.entry(doc_tf.0).or_insert(0) += 1;
            }
        }
    }
    // return the doc_ids whose count is highest
    let mut doc_ids: Vec<DocId> = doc_ids_to_term.keys().copied().collect();
    doc_ids.sort_by(|a, b| {
        let count_a = doc_ids_to_term.get(a).unwrap_or(&0);
        let count_b = doc_ids_to_term.get(b).unwrap_or(&0);
        count_b.cmp(count_a) // descending order
    });
    Some(doc_ids)
}

pub fn tokenize(query: String) -> Vec<String> {
    let s = query
        .split_ascii_whitespace()
        .map(|s| s.to_ascii_lowercase())
        .collect::<Vec<String>>();

    s
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

    fn build_index(docs: Vec<Document>) -> InvertedIndex {
        let mut index = InvertedIndex::new();
        for doc in docs {
            index.add_document(doc);
        }
        index
    }

    #[test]
    fn test_tokenize_lowercases() {
        let tokens = tokenize("Hello WORLD".to_string());
        assert_eq!(tokens, vec!["hello", "world"]);
    }

    #[test]
    fn test_tokenize_splits_whitespace() {
        let tokens = tokenize("one   two\tthree".to_string());
        assert_eq!(tokens, vec!["one", "two", "three"]);
    }

    #[test]
    fn test_search_returns_matching_docs() {
        let index = build_index(vec![
            make_doc(1, "apple banana"),
            make_doc(2, "banana cherry"),
        ]);

        let results = search(&index, "banana").unwrap();
        assert!(results.contains(&1));
        assert!(results.contains(&2));
    }

    #[test]
    fn test_search_empty_query() {
        let index = build_index(vec![make_doc(1, "hello")]);
        let results = search(&index, "").unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn test_search_no_matches() {
        let index = build_index(vec![make_doc(1, "hello world")]);
        let results = search(&index, "xyz").unwrap();
        assert!(results.is_empty());
    }

    #[test]
    fn test_search_ranks_by_match_count() {
        let index = build_index(vec![
            make_doc(1, "hello"),               // matches 1 term
            make_doc(2, "hello world"),         // matches 2 terms
            make_doc(4, "hello world foo bar"), // matches 3 terms
        ]);

        let results = search(&index, "hello world foo").unwrap();

        assert_eq!(results[0], 4); // most matches
        assert!(results.contains(&2));
        assert_eq!(results[results.len() - 1], 1); // fewest matches
    }

    #[test]
    fn test_search_empty_when_index_empty() {
        let index = InvertedIndex::new();
        let results = search(&index, "hello").unwrap();
        assert!(results.is_empty());
    }
}

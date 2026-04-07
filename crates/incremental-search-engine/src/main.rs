use clap::Parser;

mod ingestion;
mod index;
mod query;
mod types;

use ingestion::ingest_document;
use index::InvertedIndex;
use query::search;
use types::{Document};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(long)]
    upload: Option<String>,

    #[arg(long)]
    query: Option<String>,
}

fn main() {
    let args = Args::parse();

    let mut index = InvertedIndex::new();

    if let Some(path) = args.upload {
        let docs = ingest_document(&path);
        for doc in docs {
            index.add_document(doc);
        }
    }

    if let Some(q) = args.query {
        let results = search(&index, &q);
        for r in results {
            println!("{:?}", r);
        }
    }
}
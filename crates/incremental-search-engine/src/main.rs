use clap::Parser;

mod index;
mod ingestion;
mod query;
mod types;

use index::InvertedIndex;
use ingestion::ingest_document;
use query::search;

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
        if let Some(doc) = ingest_document(&path) {
            index.add_document(doc);
        }
    }

    if let Some(q) = args.query {
        if let Some(results) = search(&index, &q) {
            for doc_id in results {
                println!("{}", doc_id);
            }
        }
    }
}

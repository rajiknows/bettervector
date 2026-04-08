pub type DocId = u64;
pub type ChunkId = u64;

#[derive(Debug)]
pub struct Document {
    pub id: DocId,
    pub text: String,
}

pub struct Chunk {
    pub doc_id: DocId,
    pub chunk_id: ChunkId,
    pub text: String,
}

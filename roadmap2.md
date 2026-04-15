# Search Infrastructure Roadmap

Target: Staff/Principal Search Infrastructure Engineer (Rust)

Current: Basic inverted index (term → doc postings), simple TF-based ranking.

---

## Phase 1: Production-Ready Lexical Search

**Focus: Get BM25 working, understand scoring deeply**

- [ ] **BM25 ranking** - Replace simple TF count with BM25 (k1, b parameters)
- [ ] **Understand IDF** - Why log(N/df) matters for relevance
- [ ] **Tokenizer improvements** - Stemming (Porter), stopwords, n-grams
- [ ] **Index statistics** - Doc count, avg doc length, collection frequency
- [ ] **Query parser** - Boolean operators (AND/OR/NOT), phrase search
- [ ] **Index persistence** - Save/load inverted index to disk
- [ ] **Tests** - Offline evaluation with recall@k, MRR

**Why:** Job requires "lexical search (e.g. BM25)" - this is foundational.

---

## Phase 2: Indexing Pipeline

**Focus: Ingestion, chunking, deduplication - what the job calls "data pipelines"**

- [ ] **Text chunking** - Fixed-size chunks, sentence boundary detection
- [ ] **Incremental indexing** - Add docs without full rebuild
- [ ] **Content deduplication** - SimHash or MinHash for near-duplicate detection
- [ ] **Document canonicalization** - HTML stripping, language detection
- [ ] **Async ingestion** - Tokio for parallel document processing
- [ ] **Batch inserts** - Rayon for parallel indexing

**Why:** Job mentions "ingestion, canonicalization, chunking, incremental indexing, deduplication"

---

## Phase 3: Hybrid Retrieval

**Focus: Combine lexical + vector search (RRF, naive hybrid)**

- [ ] **Embeddings** - Generate with a model (sentence-transformers via candle-rs or ONNX)
- [ ] **Simple ANN** - Use or build basic HNSW/FAISS wrapper
- [ ] **Hybrid fusion** - RRF (Reciprocal Rank Fusion) to combine scores
- [ ] **Result deduplication** - Remove near-identical chunks from results
- [ ] **Score normalization** - MinMax or rank-based normalization across systems

**Why:** Job says "hybrid retrieval" - AI search almost always needs this.

---

## Phase 4: Ranking & Reranking

**Focus: Multi-stage pipelines, quality signals**

- [ ] **Two-stage retrieval** - Coarse (ANN) → Fine (cross-encoder reranker)
- [ ] **Quality signals** - Freshness, authority signals, link analysis
- [ ] **Learning to rank basics** - LambdaMART / BM25F concepts
- [ ] **Provenance/citations** - Attach source metadata to results
- [ ] **Offline evaluation** - NDCG, precision@k, user studies

**Why:** Job emphasizes "ranking, relevance & grounding"

---

## Phase 5: Scale & Production

**Focus: What makes search systems actually production-ready**

- [ ] **Persistence & recovery** - WAL, crash safety, index snapshots
- [ ] **Concurrency** - Arc<RwLock>, multi-threaded queries
- [ ] **Filtering** - Pre-filter and post-filter strategies
- [ ] **Memory optimization** - Quantization (PQ/SQ), memory-mapped files
- [ ] **Serving layer** - HTTP/gRPC API (Axum or tonic)
- [ ] **Observability** - Latency histograms, query logging

**Why:** "Low latency, freshness, and scale" - core job requirement.

---

## Phase 6: Advanced Topics (Nice to Have)

**Focus: Differentiating for Staff+ level**

- [ ] **HNSW from scratch** - Understanding the algorithm deeply
- [ ] **Graph storage basics** - Entity linking, adjacency lists
- [ ] **Anti-spam signals** - Quality classification basics
- [ ] **Adaptive k** - Dynamic ef_construction based on data characteristics

---

## Weekly Breakdown (12 weeks)

| Week | Focus | Deliverable |
|------|-------|-------------|
| 1-2 | BM25 + tokenizer | Search quality noticeably better |
| 3-4 | Query parser + persistence | Boolean queries work, index survives restart |
| 5-6 | Ingestion pipeline | Process 100k docs efficiently |
| 7-8 | Embeddings + ANN | Hybrid search functional |
| 9-10 | Reranking + evaluation | Results are high quality |
| 11-12 | Scale + serving | Production-ready demo |

---

## Interview Talking Points by Phase

**Phase 1:** "BM25 vs TF - why log helps" / "Impact of b parameter on long docs"

**Phase 2:** "Incremental vs batch indexing tradeoffs" / "How deduplication scales"

**Phase 3:** "RRF vs score averaging" / "When hybrid beats pure lexical"

**Phase 4:** "Two-stage vs single-stage" / "Why reranking order matters"

**Phase 5:** "Write-ahead logs for crash safety" / "Quantization accuracy tradeoff"

**Phase 6:** "HNSW construction vs search tradeoff" / "ef vs recall"

---

## Real Corpora for Testing

- Wikipedia dump (XML/JSON)
- Common Crawl subset
- Semantic Scholar (abstractions)
- OpenWebText

Store results, benchmarks, and design decisions in `/benchmarks/` and `/docs/`.

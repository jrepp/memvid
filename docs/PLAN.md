# memvid — Project Plan

**Last Updated**: 2026-02-18

## Current State

Rust library providing a single-file memory layer for AI agents. Packages documents, embeddings, search indices, and metadata into portable `.mv2` files. Core dependency for z-manuals-kb.

### Completed Milestones

- .mv2 file format: header, WAL, data segments, lex index, vec index, time index, TOC
- Full-text search via Tantivy (BM25)
- Vector search via HNSW (cosine similarity)
- CLIP image embeddings (feature flag)
- Audio transcription via Whisper (feature flag)
- AES-256-GCM encryption (feature flag)
- Write-ahead log for crash safety
- Append-only immutable frames
- Batch operations for performance
- Chronological time index

### Test Coverage

- Unit tests: `cargo test`
- Lifecycle integration test: `cargo test --test lifecycle`
- Benchmarks: `cargo bench`

## Active Work Items

### P3 — Performance

| Item | Description | Status |
|------|-------------|--------|
| Index compaction | Merge WAL segments to reduce file size | Not started |
| Concurrent reads | Allow multiple readers during writes | Not started |
| Memory-mapped I/O | Reduce memory copies for large indices | Not started |

### P3 — API Improvements

| Item | Description | Status |
|------|-------------|--------|
| Incremental indexing | Add documents without full reindex | Not started |
| Delete support | Mark frames as deleted (soft delete) | Not started |
| Metadata queries | Filter by metadata fields | Not started |

## Future Considerations

- Distributed .mv2 sharding for large knowledge bases
- Streaming ingestion API
- Python bindings (PyO3)

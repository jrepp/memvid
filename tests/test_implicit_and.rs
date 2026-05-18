//! Integration tests for implicit OR operator behavior.
//!
//! These tests verify that multi-word queries without explicit operators
//! use OR logic (not AND) for high recall, with BM25 ranking documents
//! matching more terms higher.
//!
//! # Tests
//!
//! - `test_implicit_or_recall`: Verifies that multi-word queries return
//!   documents matching ANY term, ranked by relevance
//! - `test_explicit_operators_still_work`: Ensures backward compatibility
//!   with explicit AND/OR operators

use memvid_core::{Memvid, PutOptions, SearchRequest};

#[test]
fn test_implicit_or_recall() -> memvid_core::Result<()> {
    let temp_file = std::env::temp_dir().join("test_implicit_or.mv2");
    let _ = std::fs::remove_file(&temp_file);

    let mut mem = Memvid::create(&temp_file)?;

    mem.put_bytes_with_options(
        b"Machine learning is a subset of artificial intelligence",
        PutOptions::builder().title("Doc 1: ML only").build(),
    )?;

    mem.put_bytes_with_options(
        b"Python is a popular programming language",
        PutOptions::builder().title("Doc 2: Python only").build(),
    )?;

    mem.put_bytes_with_options(
        b"Machine learning with Python is very powerful",
        PutOptions::builder()
            .title("Doc 3: Both ML and Python")
            .build(),
    )?;

    mem.commit()?;

    let results = mem.search(SearchRequest {
        query: "machine python".to_string(),
        top_k: 10,
        snippet_chars: 200,
        uri: None,
        scope: None,
        cursor: None,
        #[cfg(feature = "temporal_track")]
        temporal: None,
        as_of_frame: None,
        as_of_ts: None,
        no_sketch: false,
        acl_context: None,
        acl_enforcement_mode: memvid_core::types::AclEnforcementMode::Audit,
    })?;

    // With implicit OR, all three docs should match (each contains at least one term).
    // Doc 3 (both terms) should rank highest.
    assert!(
        results.hits.len() >= 2,
        "Query 'machine python' should match multiple docs with OR logic, got {}",
        results.hits.len()
    );
    assert!(
        results.hits[0].title.as_ref().unwrap().contains("Doc 3"),
        "Doc 3 (both terms) should rank first"
    );

    std::fs::remove_file(&temp_file)?;
    Ok(())
}

#[test]
fn test_explicit_operators_still_work() -> memvid_core::Result<()> {
    let temp_file = std::env::temp_dir().join("test_explicit_ops.mv2");
    let _ = std::fs::remove_file(&temp_file);

    let mut mem = Memvid::create(&temp_file)?;

    mem.put_bytes_with_options(
        b"Rust programming language",
        PutOptions::builder().title("Doc 1").build(),
    )?;

    mem.put_bytes_with_options(
        b"Go programming language",
        PutOptions::builder().title("Doc 2").build(),
    )?;

    mem.put_bytes_with_options(
        b"Rust and Go are both systems languages",
        PutOptions::builder().title("Doc 3").build(),
    )?;

    mem.commit()?;

    // Explicit AND — only Doc 3 matches
    let results = mem.search(SearchRequest {
        query: "Rust AND Go".to_string(),
        top_k: 10,
        snippet_chars: 200,
        uri: None,
        scope: None,
        cursor: None,
        #[cfg(feature = "temporal_track")]
        temporal: None,
        as_of_frame: None,
        as_of_ts: None,
        no_sketch: false,
        acl_context: None,
        acl_enforcement_mode: memvid_core::types::AclEnforcementMode::Audit,
    })?;

    assert_eq!(
        results.hits.len(),
        1,
        "Explicit AND should match only Doc 3"
    );

    // Explicit OR — all docs match
    let results = mem.search(SearchRequest {
        query: "Rust OR Go".to_string(),
        top_k: 10,
        snippet_chars: 200,
        uri: None,
        scope: None,
        cursor: None,
        #[cfg(feature = "temporal_track")]
        temporal: None,
        as_of_frame: None,
        as_of_ts: None,
        no_sketch: false,
        acl_context: None,
        acl_enforcement_mode: memvid_core::types::AclEnforcementMode::Audit,
    })?;

    assert!(
        results.hits.len() >= 2,
        "Explicit OR should match multiple docs"
    );

    std::fs::remove_file(&temp_file)?;
    Ok(())
}

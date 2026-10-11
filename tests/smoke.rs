//! Integration test: exercises the crate through its public API.
//!
//! Run with `mise run test`, which drives `cargo nextest`.

#[test]
fn greeting_is_not_empty() {
    assert!(!crwst::greeting().is_empty());
}

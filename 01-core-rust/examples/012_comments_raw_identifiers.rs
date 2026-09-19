//! # Comments, documentation, and raw identifiers
//!
//! Level: Beginner
//! Category: Core Rust
//!
//! ## What
//! Rust has line, block, outer-doc, and inner-doc comments. Raw identifiers use `r#`
//! when a useful external name is also a Rust keyword.
//!
//! ## Why
//! Documentation comments feed rustdoc, while raw identifiers preserve compatible names
//! at language or protocol boundaries.
//!
//! ## When to use
//! Document public APIs and use raw identifiers only when a keyword-shaped name is required.
//!
//! ## Avoid when
//! Do not use comments to explain code that can instead be made clearer by naming or structure.
//!
//! ## Prerequisites
//! Functions and modules.
//!
//! ## Related
//! Attributes, crates, rustdoc.
//!
//! ## Run
//! `cargo run -p core-rust-lessons --example 012_comments_raw_identifiers`

/// Returns a protocol field that happens to be a Rust keyword.
fn r#type() -> &'static str {
    // A line comment explains a local decision.
    "lesson"
}

fn main() {
    /* Block comments can span a short explanation. */
    println!("record type: {}", r#type());
}

#[cfg(test)]
mod tests {
    use super::r#type;

    #[test]
    fn raw_identifier_can_be_called_normally() {
        assert_eq!(r#type(), "lesson");
    }
}

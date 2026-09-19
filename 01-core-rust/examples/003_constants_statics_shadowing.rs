//! # Constants, statics, and shadowing
//!
//! Level: Beginner
//! Category: Core Rust
//!
//! ## What
//! Constants are compile-time values, immutable statics have one fixed location,
//! and shadowing creates a new binding with an existing name.
//!
//! ## Why
//! These forms express different lifetimes and intent without mutable global state.
//!
//! ## When to use
//! Use constants for named invariant values and shadowing for a transformed value.
//!
//! ## Avoid when
//! Avoid global mutable state; it complicates initialization and thread safety.
//!
//! ## Prerequisites
//! Variables and mutability.
//!
//! ## Related
//! Types, ownership, `OnceLock`.
//!
//! ## Run
//! `cargo run -p core-rust-lessons --example 003_constants_statics_shadowing`

const BITS_PER_BYTE: u8 = 8;
static PROJECT_NAME: &str = "Rust Zero to Systems";

fn main() {
    let spaces = "   ";
    let spaces = spaces.len();

    println!("{PROJECT_NAME}: {BITS_PER_BYTE} bits/byte; {spaces} spaces counted.");
}

#[cfg(test)]
mod tests {
    use super::BITS_PER_BYTE;

    #[test]
    fn constants_have_a_declared_type() {
        assert_eq!(BITS_PER_BYTE, 8);
    }
}

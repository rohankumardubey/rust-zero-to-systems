//! # Tuples, arrays, and slices
//!
//! Level: Beginner
//! Category: Core Rust
//!
//! ## What
//! Tuples combine differently typed values; arrays have fixed length; slices borrow
//! a contiguous portion without owning it.
//!
//! ## Why
//! They model fixed-size data and views without allocating a `Vec` unnecessarily.
//!
//! ## When to use
//! Use arrays for known-size data and slices in APIs that accept a view of a sequence.
//!
//! ## Avoid when
//! Use a `Vec` when the collection must grow or shrink at runtime.
//!
//! ## Prerequisites
//! Scalar types and references.
//!
//! ## Related
//! `Vec`, borrowing, pattern matching.
//!
//! ## Run
//! `cargo run -p core-rust-lessons --example 005_tuples_arrays_slices`

fn main() {
    let release = (1_u8, 98_u8, 1_u8);
    let channels = ["stable", "beta", "nightly"];
    let preview = &channels[..2];

    println!(
        "release {}.{}.{}; preview: {preview:?}",
        release.0, release.1, release.2
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn a_slice_knows_its_view_length() {
        let values = [10, 20, 30, 40];
        assert_eq!(&values[1..3], [20, 30]);
    }
}

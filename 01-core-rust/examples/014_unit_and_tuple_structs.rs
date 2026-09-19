//! # Unit and tuple structs
//!
//! Level: Beginner
//! Category: Core Rust
//!
//! ## What
//! Tuple structs give positional fields a distinct named type; unit structs carry a type-level
//! meaning without storing data.
//!
//! ## Why
//! They prevent mixing values with the same representation but different meanings.
//!
//! ## When to use
//! Use a tuple struct for a small single-value domain type and a unit struct as a marker.
//!
//! ## Avoid when
//! Prefer named fields once positions stop being obvious to readers.
//!
//! ## Prerequisites
//! Structs and methods.
//!
//! ## Related
//! Newtypes, traits, zero-sized types.
//!
//! ## Run
//! `cargo run -p core-rust-lessons --example 014_unit_and_tuple_structs`

#[derive(Debug, PartialEq, Eq)]
struct Milliseconds(u64);

#[derive(Debug, Default)]
struct JsonFormat;

fn main() {
    let timeout = Milliseconds(250);
    let format = JsonFormat;
    println!("timeout: {} ms; format: {format:?}", timeout.0);
}

#[cfg(test)]
mod tests {
    use super::{JsonFormat, Milliseconds};

    #[test]
    fn tuple_struct_keeps_its_domain_type() {
        assert_eq!(Milliseconds(5), Milliseconds(5));
        assert_eq!(std::mem::size_of::<JsonFormat>(), 0);
    }
}

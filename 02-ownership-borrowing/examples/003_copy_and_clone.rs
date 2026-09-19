//! # `Copy` and `Clone`
//!
//! Level: Beginner
//! Category: Ownership and memory
//!
//! ## What
//! `Copy` types duplicate implicitly; `Clone` performs an explicit type-defined duplication.
//!
//! ## Why
//! The distinction makes potentially expensive or semantically meaningful duplication visible.
//!
//! ## When to use
//! Rely on `Copy` for small value-like types; call `clone` when independent owned data is required.
//!
//! ## Avoid when
//! Do not derive or call `Clone` as a default escape hatch for unclear ownership.
//!
//! ## Prerequisites
//! Moves and scalar types.
//!
//! ## Related
//! Ownership, allocation, reference counting.
//!
//! ## Run
//! `cargo run -p ownership-borrowing-lessons --example 003_copy_and_clone`

fn main() {
    let port: u16 = 8080;
    let backup_port = port;

    let primary = String::from("primary");
    let replica = primary.clone();

    println!("{port}/{backup_port}; {primary}/{replica}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn clone_makes_independent_owned_text() {
        let source = String::from("read");
        let mut copy = source.clone();
        copy.push_str("-only");

        assert_eq!(source, "read");
        assert_eq!(copy, "read-only");
    }
}

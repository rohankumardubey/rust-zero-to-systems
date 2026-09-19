//! # Modules, visibility, and paths
//!
//! Level: Beginner
//! Category: Core Rust
//!
//! ## What
//! Modules group items into namespaces; `pub` exposes an item through its module path.
//!
//! ## Why
//! Module boundaries let a crate present a small public API while hiding implementation details.
//!
//! ## When to use
//! Use modules whenever names or responsibilities form a coherent boundary.
//!
//! ## Avoid when
//! Do not make every item public; expose only what callers need.
//!
//! ## Prerequisites
//! Functions and structs.
//!
//! ## Related
//! Crates, packages, `use`, privacy, documentation comments.
//!
//! ## Run
//! `cargo run -p core-rust-lessons --example 010_modules_visibility_paths`

mod greeting {
    /// Formats a public greeting without exposing the internal punctuation constant.
    pub fn format(name: &str) -> String {
        format!("Hello, {name}{PUNCTUATION}")
    }

    const PUNCTUATION: char = '!';
}

fn main() {
    println!("{}", greeting::format("modules"));
}

#[cfg(test)]
mod tests {
    use super::greeting;

    #[test]
    fn public_path_reaches_the_module_api() {
        assert_eq!(greeting::format("Rust"), "Hello, Rust!");
    }
}

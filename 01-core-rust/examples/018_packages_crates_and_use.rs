//! # Packages, crates, and `use`
//!
//! Level: Beginner
//! Category: Core Rust
//!
//! ## What
//! A package is described by `Cargo.toml`; it can contain library and binary crates. `use`
//! brings a public path into the current scope.
//!
//! ## Why
//! Cargo gives related crates one dependency and build configuration, while imports keep call
//! sites readable without erasing the original public API.
//!
//! ## When to use
//! Import the specific public items that make a module’s dependencies clear.
//!
//! ## Avoid when
//! Avoid glob imports in instructional or library code when they obscure where a name came from.
//!
//! ## Prerequisites
//! Modules, visibility, and functions.
//!
//! ## Related
//! Workspaces, `pub use`, Cargo manifests.
//!
//! ## Run
//! `cargo run -p core-rust-lessons --example 018_packages_crates_and_use`

use core_rust_lessons::greeting;

fn main() {
    println!("{}", greeting());
}

#[cfg(test)]
mod tests {
    use core_rust_lessons::greeting;

    #[test]
    fn binary_crate_can_import_its_package_library_crate() {
        assert_eq!(greeting(), "Hello, Rust systems!");
    }
}

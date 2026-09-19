//! # Hello, world!
//!
//! Level: Beginner
//! Category: Core Rust
//!
//! ## What
//! A minimal executable with Rust's `main` entry point and `println!` macro.
//!
//! ## Why
//! It verifies the toolchain and introduces compilation, macros, and stdout.
//!
//! ## When to use
//! Use this shape for the smallest binary experiments.
//!
//! ## Avoid when
//! Use tests or library examples when behavior needs reusable APIs.
//!
//! ## Prerequisites
//! A stable Rust toolchain.
//!
//! ## Related
//! Functions, macros, formatting.
//!
//! ## Run
//! `cargo run -p core-rust-lessons --example 001_hello_world`

fn main() {
    println!("{}", core_rust_lessons::greeting());
}

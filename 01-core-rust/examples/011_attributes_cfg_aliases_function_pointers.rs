//! # Attributes, conditional compilation, aliases, and function pointers
//!
//! Level: Beginner
//! Category: Core Rust
//!
//! ## What
//! Attributes configure items, `cfg!` queries compile-time configuration, aliases name
//! types, and `fn(...) -> ...` names a function-pointer type.
//!
//! ## Why
//! These tools make platform choices and callable contracts explicit without runtime dispatch.
//!
//! ## When to use
//! Use aliases for meaningful domain names and function pointers for plain function callbacks.
//!
//! ## Avoid when
//! Prefer generics or closures when callbacks need captured state.
//!
//! ## Prerequisites
//! Functions, scalar types, modules.
//!
//! ## Related
//! Conditional compilation, closures, traits, the never type.
//!
//! ## Run
//! `cargo run -p core-rust-lessons --example 011_attributes_cfg_aliases_function_pointers`

type Port = u16;
type Operation = fn(i32, i32) -> i32;

#[must_use]
fn add(left: i32, right: i32) -> i32 {
    left + right
}

fn main() {
    let port: Port = 8080;
    let operation: Operation = add;
    let target_family = if cfg!(target_family = "unix") {
        "unix"
    } else {
        "non-unix"
    };

    println!("{target_family} service on {port}: {}", operation(20, 22));
}

#[cfg(test)]
mod tests {
    use super::{Operation, add};

    #[test]
    fn function_pointer_invokes_a_plain_function() {
        let operation: Operation = add;
        assert_eq!(operation(20, 22), 42);
    }
}

//! # Scalar types, literals, and operators
//!
//! Level: Beginner
//! Category: Core Rust
//!
//! ## What
//! Integers, floating-point values, booleans, and characters are scalar values.
//!
//! ## Why
//! Explicit numeric types make range, precision, and representation choices clear.
//!
//! ## When to use
//! Choose the smallest type that correctly models the domain after measuring needs.
//!
//! ## Avoid when
//! Do not use floating point where exact decimal arithmetic is a requirement.
//!
//! ## Prerequisites
//! Variables and mutability.
//!
//! ## Related
//! Tuples, conversions, overflow behavior.
//!
//! ## Run
//! `cargo run -p core-rust-lessons --example 004_scalar_types_literals_operators`

fn main() {
    let signed: i16 = -42;
    let hexadecimal = 0xff_u16;
    let ratio: f64 = 3.0 / 2.0;
    let is_stable = true;
    let ferris = '🦀';

    println!("{signed}, {hexadecimal}, {ratio}, {is_stable}, {ferris}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn arithmetic_uses_the_binding_type() {
        let total: u16 = 250 + 5;
        assert_eq!(total, 255);
    }
}

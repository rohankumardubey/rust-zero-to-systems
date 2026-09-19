//! # Conversions and casts
//!
//! Level: Beginner
//! Category: Core Rust
//!
//! ## What
//! `as` performs an explicit primitive cast; `From` and `TryFrom` express infallible and
//! fallible conversions through traits.
//!
//! ## Why
//! Conversion syntax makes potential representation or range changes visible at the call site.
//!
//! ## When to use
//! Use `TryFrom` when a value may not fit; use `From` for lossless, unsurprising conversions.
//!
//! ## Avoid when
//! Do not use `as` to silently truncate a value when validation is required.
//!
//! ## Prerequisites
//! Scalar types and `Result` vocabulary.
//!
//! ## Related
//! Traits, error handling, numeric overflow.
//!
//! ## Run
//! `cargo run -p core-rust-lessons --example 013_conversions_and_casts`

fn checked_port(value: u32) -> Result<u16, std::num::TryFromIntError> {
    u16::try_from(value)
}

fn main() {
    let byte: u8 = 42;
    let wider = u16::from(byte);
    let crab_code_point = '🦀' as u32;

    println!(
        "{wider}; crab code point: {crab_code_point}; port: {:?}",
        checked_port(8080)
    );
}

#[cfg(test)]
mod tests {
    use super::checked_port;

    #[test]
    fn fallible_conversion_rejects_out_of_range_values() {
        assert_eq!(checked_port(8080), Ok(8080));
        assert!(checked_port(u32::from(u16::MAX) + 1).is_err());
    }
}

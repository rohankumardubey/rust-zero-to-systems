//! Executable lessons for core Rust syntax and semantics.
//!
//! Each focused lesson lives in `examples/` and is runnable on its own.

/// Returns the repository's first greeting.
#[must_use]
pub const fn greeting() -> &'static str {
    "Hello, Rust systems!"
}

#[cfg(test)]
mod tests {
    use super::greeting;

    #[test]
    fn greeting_is_stable_for_examples() {
        assert_eq!(greeting(), "Hello, Rust systems!");
    }
}

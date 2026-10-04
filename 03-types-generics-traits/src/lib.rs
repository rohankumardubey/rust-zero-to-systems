//! Executable lessons about generics, traits, and Rust's type system.

/// Returns the topic represented by this lesson crate.
#[must_use]
pub const fn subject() -> &'static str {
    "generics, traits, and the type system"
}

#[cfg(test)]
mod tests {
    use super::subject;

    #[test]
    fn subject_is_stable() {
        assert_eq!(subject(), "generics, traits, and the type system");
    }
}

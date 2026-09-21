//! Executable lessons about lifetime relationships and smart pointers.

/// Returns the subject represented by this lesson crate.
#[must_use]
pub const fn subject() -> &'static str {
    "lifetimes and smart pointers"
}

#[cfg(test)]
mod tests {
    use super::subject;

    #[test]
    fn subject_is_stable() {
        assert_eq!(subject(), "lifetimes and smart pointers");
    }
}

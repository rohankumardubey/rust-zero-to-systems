//! Executable lessons about ownership, borrowing, and resource lifetimes.

/// Returns the phase name used by the examples.
#[must_use]
pub const fn phase_name() -> &'static str {
    "ownership and borrowing"
}

#[cfg(test)]
mod tests {
    use super::phase_name;

    #[test]
    fn phase_name_is_stable() {
        assert_eq!(phase_name(), "ownership and borrowing");
    }
}

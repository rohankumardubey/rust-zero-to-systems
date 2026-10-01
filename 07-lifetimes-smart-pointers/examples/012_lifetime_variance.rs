//! # Lifetime variance
//!
//! Level: Advanced
//! Category: Lifetimes and smart pointers
//!
//! ## What
//! Covariance allows a reference valid for longer to be used where a shorter borrow is required.
//!
//! ## Why
//! Variance explains which lifetime substitutions are safe through nested types and why some
//! generic containers restrict them.
//!
//! ## When to use
//! Use this model when designing borrowed generic types or interpreting compiler lifetime errors.
//!
//! ## Avoid when
//! Do not rely on variance to make a value live longer; it only permits shortening a relationship.
//!
//! ## Prerequisites
//! References, lifetime bounds, and generic types.
//!
//! ## Related
//! `Cell`, mutable references, subtyping, `PhantomData`.
//!
//! ## Run
//! `cargo run -p lifetimes-smart-pointers-lessons --example 012_lifetime_variance`

fn shorten<'long: 'short, 'short>(value: &'long str, _scope: &'short ()) -> &'short str {
    value
}

fn main() {
    let text = String::from("a borrow can be shortened");
    let scope = ();
    println!("{}", shorten(&text, &scope));
}

#[cfg(test)]
mod tests {
    use super::shorten;

    #[test]
    fn covariant_shared_reference_can_be_shortened() {
        let text = String::from("valid for the enclosing scope");
        let scope = ();
        assert_eq!(shorten(&text, &scope), "valid for the enclosing scope");
    }
}

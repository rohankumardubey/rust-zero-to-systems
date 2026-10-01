//! # Higher-ranked trait bounds
//!
//! Level: Advanced
//! Category: Lifetimes and smart pointers
//!
//! ## What
//! `for<'a>` requires a callable to work for every suitable lifetime, rather than one fixed borrow.
//!
//! ## Why
//! The bound lets a generic API accept functions that borrow inputs without choosing their lifetime.
//!
//! ## When to use
//! Use higher-ranked bounds for callbacks that must accept a fresh borrow of any lifetime.
//!
//! ## Avoid when
//! Prefer inferred bounds when the API does not need to state a universally quantified lifetime.
//!
//! ## Prerequisites
//! Function traits, closures, and lifetime annotations.
//!
//! ## Related
//! `Fn`, trait objects, lifetime elision, async callbacks.
//!
//! ## Run
//! `cargo run -p lifetimes-smart-pointers-lessons --example 013_higher_ranked_trait_bounds`

fn apply_to_borrow<F>(callback: F, input: &str) -> String
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    callback(input).to_owned()
}

fn main() {
    let text = String::from("borrowed callback");
    let result = apply_to_borrow(str::trim, &text);
    println!("{result}");
}

#[cfg(test)]
mod tests {
    use super::apply_to_borrow;

    #[test]
    fn callback_accepts_a_borrow_with_the_callers_lifetime() {
        assert_eq!(apply_to_borrow(str::trim, "  hello  "), "hello");
    }
}

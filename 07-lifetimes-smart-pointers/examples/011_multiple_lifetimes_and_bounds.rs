//! # Multiple lifetimes and outlives bounds
//!
//! Level: Intermediate
//! Category: Lifetimes and smart pointers
//!
//! ## What
//! Independent lifetime parameters describe independent borrows; an outlives bound states that
//! one borrow is valid for at least as long as another relationship requires.
//!
//! ## Why
//! Naming only the relationships an API needs avoids unnecessarily tying unrelated inputs together.
//!
//! ## When to use
//! Use multiple lifetimes when a returned value or stored type relates to more than one borrow.
//!
//! ## Avoid when
//! Do not add lifetime parameters that do not express a real relationship.
//!
//! ## Prerequisites
//! Lifetime annotations, borrowed structs, and lifetime elision.
//!
//! ## Related
//! Subtyping, variance, higher-ranked trait bounds.
//!
//! ## Run
//! `cargo run -p lifetimes-smart-pointers-lessons --example 011_multiple_lifetimes_and_bounds`

#[derive(Debug, PartialEq, Eq)]
struct Excerpt<'text, 'source>
where
    'text: 'source,
{
    text: &'text str,
    source: &'source str,
}

impl<'text, 'source> Excerpt<'text, 'source>
where
    'text: 'source,
{
    fn text_during_source(&'source self) -> &'source str {
        self.text
    }
}

fn main() {
    let document = String::from("Rust systems notes");
    let source = String::from("lesson 1");
    let excerpt = Excerpt {
        text: &document,
        source: &source,
    };
    println!("{} ({})", excerpt.text_during_source(), excerpt.source);
}

#[cfg(test)]
mod tests {
    use super::Excerpt;

    #[test]
    fn independent_borrows_can_have_distinct_relationships() {
        let document = String::from("borrowing relationships");
        let source = String::from("chapter");
        let excerpt = Excerpt {
            text: &document,
            source: &source,
        };

        assert_eq!(excerpt.text_during_source(), "borrowing relationships");
        assert_eq!(excerpt.source, "chapter");
    }
}

//! # Struct lifetime parameters
//!
//! Level: Intermediate
//! Category: Lifetimes and smart pointers
//!
//! ## What
//! A struct that stores a reference names the lifetime relationship of that stored borrow.
//!
//! ## Why
//! The type prevents the struct from outliving the value it refers to.
//!
//! ## When to use
//! Use a lifetime parameter when a data type stores borrowed data rather than owning it.
//!
//! ## Avoid when
//! Prefer owned fields when the value must be retained independently of its caller.
//!
//! ## Prerequisites
//! Lifetime annotations and structs.
//!
//! ## Related
//! `String` versus `&str`, ownership, self-referential types.
//!
//! ## Run
//! `cargo run -p lifetimes-smart-pointers-lessons --example 002_struct_lifetimes`

#[derive(Debug, PartialEq, Eq)]
struct Label<'a> {
    text: &'a str,
}

impl Label<'_> {
    fn is_empty(&self) -> bool {
        self.text.is_empty()
    }
}

fn main() {
    let title = String::from("storage");
    let label = Label { text: &title };
    println!("{label:?}; empty: {}", label.is_empty());
}

#[cfg(test)]
mod tests {
    use super::Label;

    #[test]
    fn struct_cannot_outlive_its_borrowed_text() {
        let title = String::from("networking");
        let label = Label { text: &title };
        assert!(!label.is_empty());
    }
}

//! # Trait definitions, implementations, and defaults
//!
//! Level: Beginner
//! Category: Generics and traits
//!
//! ## What
//! A trait names behavior that multiple types can implement; default methods provide reusable
//! behavior that an implementation may override.
//!
//! ## Why
//! Traits let callers depend on capabilities instead of one concrete type.
//!
//! ## When to use
//! Define a trait for behavior that has meaningful implementations across distinct types.
//!
//! ## Avoid when
//! Keep behavior inherent when only one type needs it and no abstraction is useful.
//!
//! ## Prerequisites
//! Structs, methods, and associated functions.
//!
//! ## Related
//! Trait bounds, associated types, dynamic dispatch.
//!
//! ## Run
//! `cargo run -p types-generics-traits-lessons --example 002_trait_definitions_and_defaults`

trait Summary {
    fn title(&self) -> &str;

    fn summarize(&self) -> String {
        format!("Read more: {}", self.title())
    }
}

struct Article {
    headline: String,
}

struct ReleaseNote {
    version: String,
}

impl Summary for Article {
    fn title(&self) -> &str {
        &self.headline
    }
}

impl Summary for ReleaseNote {
    fn title(&self) -> &str {
        &self.version
    }

    fn summarize(&self) -> String {
        format!("Release {} is available", self.version)
    }
}

fn main() {
    let article = Article {
        headline: String::from("Borrowing without ownership"),
    };
    let release = ReleaseNote {
        version: String::from("2.4"),
    };
    println!("{}", article.summarize());
    println!("{}", release.summarize());
}

#[cfg(test)]
mod tests {
    use super::{Article, ReleaseNote, Summary};

    #[test]
    fn implementations_can_use_or_override_default_behavior() {
        let article = Article {
            headline: String::from("A shared abstraction"),
        };
        let release = ReleaseNote {
            version: String::from("1.0"),
        };

        assert_eq!(article.summarize(), "Read more: A shared abstraction");
        assert_eq!(release.summarize(), "Release 1.0 is available");
    }
}

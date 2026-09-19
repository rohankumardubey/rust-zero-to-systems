//! # Structs, enums, methods, and associated functions
//!
//! Level: Beginner
//! Category: Core Rust
//!
//! ## What
//! Structs name related fields; enums model one of several variants; `impl` blocks
//! attach methods and associated functions.
//!
//! ## Why
//! These types encode valid domain states more precisely than unrelated primitives.
//!
//! ## When to use
//! Use a struct for data that exists together and an enum for mutually exclusive states.
//!
//! ## Avoid when
//! Do not use an enum variant with unrelated payloads merely to avoid defining a focused type.
//!
//! ## Prerequisites
//! Functions, pattern matching.
//!
//! ## Related
//! Traits, visibility, `Option` and `Result`.
//!
//! ## Run
//! `cargo run -p core-rust-lessons --example 009_structs_enums_methods`

#[derive(Debug, PartialEq, Eq)]
struct Build {
    name: String,
    state: BuildState,
}

#[derive(Debug, PartialEq, Eq)]
enum BuildState {
    Queued,
    Finished { warnings: u8 },
}

impl Build {
    fn queued(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            state: BuildState::Queued,
        }
    }

    fn finish(&mut self, warnings: u8) {
        self.state = BuildState::Finished { warnings };
    }

    fn is_finished(&self) -> bool {
        matches!(self.state, BuildState::Finished { .. })
    }
}

fn main() {
    let mut build = Build::queued("book");
    build.finish(0);
    println!("{} finished: {}", build.name, build.is_finished());
}

#[cfg(test)]
mod tests {
    use super::Build;

    #[test]
    fn method_updates_the_modeled_state() {
        let mut build = Build::queued("check");
        assert!(!build.is_finished());
        build.finish(2);
        assert!(build.is_finished());
    }
}

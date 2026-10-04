//! # Static and dynamic dispatch
//!
//! Level: Intermediate
//! Category: Generics and traits
//!
//! ## What
//! A generic bound normally selects an implementation at compile time; a trait object such as
//! `&dyn Render` selects an implementation through a runtime vtable.
//!
//! ## Why
//! Rust offers both abstraction styles so APIs can choose compile-time specialization or runtime
//! heterogeneity.
//!
//! ## When to use
//! Use generics for a known set of concrete types and trait objects when one value must hold or
//! accept different implementors behind a common interface.
//!
//! ## Avoid when
//! Do not use a trait object where a concrete type is clearer; do not claim a speed advantage
//! without measuring the actual workload.
//!
//! ## Prerequisites
//! Traits, implementations, and generic bounds.
//!
//! ## Related
//! Monomorphization, dyn compatibility, fat pointers, vtables.
//!
//! ## Run
//! `cargo run -p types-generics-traits-lessons --example 005_static_and_dynamic_dispatch`

trait Render {
    fn render(&self) -> String;
}

struct Button(String);

impl Render for Button {
    fn render(&self) -> String {
        format!("[{}]", self.0)
    }
}

struct Label(String);

impl Render for Label {
    fn render(&self) -> String {
        self.0.clone()
    }
}

fn render_static<T: Render>(component: &T) -> String {
    component.render()
}

fn render_dynamic(component: &dyn Render) -> String {
    component.render()
}

fn main() {
    let button = Button(String::from("save"));
    let label = Label(String::from("status: ready"));
    println!("{}", render_static(&button));

    let components: [&dyn Render; 2] = [&button, &label];
    for component in components {
        println!("{}", render_dynamic(component));
    }
}

#[cfg(test)]
mod tests {
    use super::{Button, Label, render_dynamic, render_static};

    #[test]
    fn both_dispatch_forms_call_the_implemented_behavior() {
        let button = Button(String::from("save"));
        let label = Label(String::from("ready"));
        assert_eq!(render_static(&button), "[save]");
        assert_eq!(render_dynamic(&label), "ready");
    }
}

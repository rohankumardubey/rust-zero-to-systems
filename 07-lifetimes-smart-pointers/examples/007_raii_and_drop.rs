//! # RAII and `Drop`
//!
//! Level: Intermediate
//! Category: Lifetimes and smart pointers
//!
//! ## What
//! RAII ties resource cleanup to ownership. The `Drop` trait runs when an owned value leaves scope.
//!
//! ## Why
//! Cleanup is deterministic on normal scope exit and remains coupled to the resource owner.
//!
//! ## When to use
//! Implement `Drop` for types that own resources needing release or finalization.
//!
//! ## Avoid when
//! Do not rely on `Drop` for failures that callers must observe; expose an explicit fallible close
//! operation when the outcome matters.
//!
//! ## Prerequisites
//! Ownership, scopes, and smart pointers.
//!
//! ## Related
//! Partial moves, file handles, mutex guards.
//!
//! ## Run
//! `cargo run -p lifetimes-smart-pointers-lessons --example 007_raii_and_drop`

use std::cell::RefCell;
use std::rc::Rc;

struct AuditGuard {
    name: &'static str,
    events: Rc<RefCell<Vec<&'static str>>>,
}

impl Drop for AuditGuard {
    fn drop(&mut self) {
        self.events.borrow_mut().push(self.name);
    }
}

fn main() {
    let events = Rc::new(RefCell::new(Vec::new()));
    {
        let _guard = AuditGuard {
            name: "connection closed",
            events: Rc::clone(&events),
        };
    }
    println!("{:?}", events.borrow());
}

#[cfg(test)]
mod tests {
    use super::{AuditGuard, Rc, RefCell};

    #[test]
    fn drop_runs_when_owner_leaves_scope() {
        let events = Rc::new(RefCell::new(Vec::new()));
        {
            let _guard = AuditGuard {
                name: "released",
                events: Rc::clone(&events),
            };
        }

        assert_eq!(&*events.borrow(), &["released"]);
    }
}

//! # `RefCell<T>`
//!
//! Level: Intermediate
//! Category: Lifetimes and smart pointers
//!
//! ## What
//! `RefCell<T>` enforces Rust's borrowing rules at runtime instead of compile time.
//!
//! ## Why
//! It supports controlled interior mutability when static borrowing cannot express a valid design.
//!
//! ## When to use
//! Use it for single-threaded mutation behind shared ownership with a clearly scoped invariant.
//!
//! ## Avoid when
//! Do not use it to postpone a borrow-design problem; invalid overlapping borrows panic at runtime.
//!
//! ## Prerequisites
//! Mutable borrows and `Rc`.
//!
//! ## Related
//! `Cell`, `Mutex`, `UnsafeCell`, interior mutability.
//!
//! ## Run
//! `cargo run -p lifetimes-smart-pointers-lessons --example 006_refcell`

use std::cell::RefCell;

fn main() {
    let retries = RefCell::new(0_u8);
    {
        let mut write = retries.borrow_mut();
        *write += 1;
    }
    println!("retries: {}", retries.borrow());
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;

    #[test]
    fn mutation_is_checked_at_runtime_with_scoped_borrow() {
        let values = RefCell::new(vec![1]);
        values.borrow_mut().push(2);
        assert_eq!(&*values.borrow(), &[1, 2]);
    }
}

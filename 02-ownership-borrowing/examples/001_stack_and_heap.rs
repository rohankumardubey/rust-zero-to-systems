//! # Stack and heap values
//!
//! Level: Beginner
//! Category: Ownership and memory
//!
//! ## What
//! Fixed-size values such as `u32` can live directly in stack frames; `String` owns a
//! growable UTF-8 buffer whose elements are stored on the heap.
//!
//! ## Why
//! Rust tracks the owner of heap-backed resources so cleanup happens exactly once.
//!
//! ## When to use
//! Use fixed-size values directly and heap-backed containers when data must grow at runtime.
//!
//! ## Avoid when
//! Do not infer performance from storage location alone; measure real workloads.
//!
//! ## Prerequisites
//! Scalar types, `String`, and functions.
//!
//! ## Related
//! Moves, `Drop`, `Vec`, smart pointers.
//!
//! ## Run
//! `cargo run -p ownership-borrowing-lessons --example 001_stack_and_heap`

fn main() {
    let retry_limit: u32 = 3;
    let service_name = String::from("indexer");

    println!(
        "{service_name}: retry limit {retry_limit}; text length {} bytes",
        service_name.len()
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn string_owns_growable_text() {
        let mut name = String::from("api");
        name.push_str("-server");
        assert_eq!(name, "api-server");
    }
}

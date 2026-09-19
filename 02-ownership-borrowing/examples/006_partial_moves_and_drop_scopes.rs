//! # Partial moves and drop scopes
//!
//! Level: Intermediate
//! Category: Ownership and memory
//!
//! ## What
//! Moving one non-`Copy` field out of a struct partially moves the struct; values are dropped
//! when their scope ends unless ownership moved elsewhere.
//!
//! ## Why
//! Rust can release each resource precisely while preventing use of an invalid partially moved value.
//!
//! ## When to use
//! Destructure when only selected fields should be transferred to another owner.
//!
//! ## Avoid when
//! Avoid partial moves when later code needs the entire original struct; borrow fields instead.
//!
//! ## Prerequisites
//! Moves and structs.
//!
//! ## Related
//! `Drop`, destructuring, RAII.
//!
//! ## Run
//! `cargo run -p ownership-borrowing-lessons --example 006_partial_moves_and_drop_scopes`

struct ConnectionSettings {
    host: String,
    port: u16,
}

fn main() {
    let settings = ConnectionSettings {
        host: String::from("localhost"),
        port: 5432,
    };
    let ConnectionSettings { host, port } = settings;

    {
        let endpoint = format!("{host}:{port}");
        println!("connecting to {endpoint}");
    }
    // `endpoint` is dropped here; `host` remains owned by this outer scope.
    println!("host remains available: {host}");
}

#[cfg(test)]
mod tests {
    #[test]
    fn destructuring_can_move_one_owned_field_and_copy_another() {
        let settings = super::ConnectionSettings {
            host: String::from("db"),
            port: 5432,
        };
        let super::ConnectionSettings { host, port } = settings;

        assert_eq!(host, "db");
        assert_eq!(port, 5432);
    }
}

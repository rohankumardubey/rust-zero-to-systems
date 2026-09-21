# Smart pointers and interior mutability

`Box<T>` provides one heap owner and is the natural tool for recursive structures. `Rc<T>` allows several single-threaded owners; `Weak<T>` is a non-owning observation that can fail to upgrade after the allocation is released, which prevents a back reference from keeping a cycle alive.

`RefCell<T>` moves borrow checking to runtime. Its temporary borrow guards must obey the same shared-versus-exclusive rule; violations panic. It is useful when a valid single-threaded design needs interior mutation, not as a substitute for understanding ownership.

```rust
{{#include ../../../07-lifetimes-smart-pointers/examples/004_box.rs}}
```

```rust
{{#include ../../../07-lifetimes-smart-pointers/examples/005_rc_and_weak.rs}}
```

```rust
{{#include ../../../07-lifetimes-smart-pointers/examples/006_refcell.rs}}
```

`Arc`, `Mutex`, and `RwLock` belong to the concurrency sequence because their main tradeoffs involve cross-thread sharing and synchronization.


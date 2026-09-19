# Async Rust baseline

Primary baselines: Rust Reference async expressions, stable `Future`/task/pin APIs, the Async Book, and official Tokio documentation.

Audit: async functions/blocks/await; future/poll/context/waker; pin/unpin; executor/reactor/scheduler; cooperative tasks and joins/select; cancellation safety; time; channel families, semaphore/notify; stream/sink concepts; backpressure/shutdown; blocking boundaries; `Send`, non-`Send`, and `'static` tasks; tiny executor; Tokio runtime behavior.

Status: planned flagship Phase 7.


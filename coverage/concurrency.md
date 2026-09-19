# Concurrency baseline

Primary baselines: stable `std::thread`, `std::sync`, and atomic documentation plus official maintained-library docs when adopted.

Audit: thread lifecycle/scopes/parking/TLS; `Send`/`Sync`/`Arc`; mutex/rwlock/condvar/barrier/once families; std and ecosystem channels/backpressure; atomic operations/CAS/fences/orderings/happens-before; contention, deadlock, starvation, false sharing; lock-/wait-free and ABA/reclamation concepts; work stealing; Rayon/Crossbeam/parking_lot.

Status: planned flagship Phase 6.


# Coverage audits

`manifest.toml` is the machine-readable inventory. The Markdown files in this directory are authoritative-baseline audit checklists that prevent the inventory from being mistaken for exhaustive coverage.

The checker validates declared records and computes their completion ratio:

```bash
cargo run -p coverage-check -- coverage/manifest.toml
```

Audit pages use three states: not inventoried, inventoried/in progress, and reconciled. Phase 19 is the systematic reconciliation pass against the linked primary documentation.


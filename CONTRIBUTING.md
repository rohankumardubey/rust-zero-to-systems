# Contributing

Start with `AGENTS.md`, `docs/PROJECT_SPEC.md`, and the active row in `docs/PHASES.md`.

## Lesson checklist

- Pick or add a unique concept ID in `coverage/manifest.toml`.
- Use one focused, descriptively named source file per concept when practical.
- Add the standard educational header and exact run command.
- Add original book prose and include the real source instead of copying it.
- Link prerequisites and related concepts using declared manifest IDs.
- Add tests or an appropriate compile-fail fixture.
- Update book navigation and the relevant coverage baseline.
- Run formatting, workspace checks/tests/Clippy, coverage validation, and mdBook.

Do not mark TODO-only content complete. Do not add intentionally broken examples to the default workspace. Label nightly, external-service, target-specific, and hardware-specific material clearly.

## Dependencies

Prefer the standard library for lessons it can express. For third-party crates, document why the crate is maintained, relevant, and appropriately scoped; update the ecosystem review when it is a significant teaching dependency.


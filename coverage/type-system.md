# Type-system baseline

Primary baselines: Rust Reference types/traits/generics/coercions chapters, Rust Book, and stable std trait documentation.

Audit: generic items and const generics; trait definitions/implementations/bounds; associated items and GATs; supertraits/blanket impls/coherence/orphan rules; dispatch and dyn compatibility; auto/marker traits; closure traits; conversion/borrowing/deref traits; `Sized`/DSTs; `impl Trait`; coercions/fat pointers/vtables; monomorphization; `PhantomData`; variance; typestate.

Status: Phase 4 is in progress. The initial manifest-backed tranche covers generic functions/types, trait definitions and defaults, bounds/`where`, associated types, static and dynamic dispatch, and `Into`/`AsRef`. Remaining items include generic methods and enums, supertraits, blanket impls/coherence/orphan rules, associated constants, dyn compatibility details, auto/marker traits, `Send`/`Sync`/`Sized`/`?Sized`, closure traits, conversion/borrowing/deref traits, const generics, GATs, `impl Trait`, DSTs/coercions/fat pointers/vtables, monomorphization, `PhantomData`, variance, and typestate. Collections and iterators share Phase 4 but have separate lessons and coverage pages as the tranche expands.

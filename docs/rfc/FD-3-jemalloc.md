# FD-3: jemalloc on default and release builds

Status: draft

## Summary

Upstream `fd` enables `use-jemalloc` by default. This fork's `default` feature list is empty, and `.github/workflows/release.yml` does not pass `--features use-jemalloc`. Decide whether the static musl release and ordinary `cargo build` should link jemalloc again.

## Motivation

The lexopt commit removed the `completions` feature because it depended on `clap_complete`. The same edit set `default = []`, which also dropped `use-jemalloc`. Nothing in the static-linking commits says that was required for musl.

`cargo build` on Linux and the release workflow therefore use the system allocator. macOS already excludes jemalloc in `Cargo.toml` and `src/main.rs`. Upstream's changelog says musl builds do not benefit from jemalloc. That claim was not remeasured here.

## Design

No design is settled. Linking jemalloc into the static musl binary can fail the release job's `INTERP` and `NEEDED` checks, or it can pass. That link was not run in this review.

## Alternatives considered

- Put `use-jemalloc` back in `default`, and let the release `cargo build` pick it up. Re-run the release ELF check. If the musl link gains a dynamic dependency, do not ship it.
- Leave the release command without jemalloc, and pass `--features use-jemalloc` only for non-musl builds. Document that the published Linux binaries use the system allocator.
- Leave both paths on the system allocator and delete the unused `use-jemalloc` feature. This drops a switch upstream still has.

## Open questions

- Should the static musl artifacts link jemalloc if, and only if, `readelf` still shows no interpreter and no `NEEDED` entries?
- Should a default `cargo build` on Linux glibc enable jemalloc even when the musl release does not?

## Decision

Unset. Recommendation: restore `use-jemalloc` for the default Linux glibc build. Keep it off the musl release until a release-profile link passes the existing ELF check.

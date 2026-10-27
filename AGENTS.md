# fd

Fork of sharkdp/fd. It finds files. This tree does not build for Windows. The CLI parser is lexopt.

## Commands

- `cargo test --locked` runs the unit tests and `tests/tests.rs`.
- `cargo build --locked --bin fd` builds the debug binary.
- The release binary is `cargo build --locked --profile dist --bin fd` with the linker flags in `.github/workflows/release.yml`.

`rust-toolchain.toml` pins `nightly-2026-09-15`. The test and release workflows pass `toolchain: stable` to `dtolnay/rust-toolchain`, which overrides that file.

## Invariants

- `lexopt` is the only CLI parser. Do not add `clap`, `clap_complete`, `structopt`, `argh`, or `pico-args`.
- Option spellings live in `src/cli/command.rs`. `fd -h` and `fd --help` both render that list. `doc/fd.1` is a second copy. Update it when a spelling changes.
- The default thread count is `available_parallelism`, capped at 64 (`default_num_threads` in `src/cli/fd.rs`).
- The build embeds `FD_GIT_SHA` from `git rev-parse --short=12 HEAD`, or from `FD_GIT_SHA_OVERRIDE` when that variable is non-empty hexadecimal. With neither a git checkout nor the override, the build fails.
- Default Cargo features are empty. `use-jemalloc` is off unless requested. macOS does not link jemalloc. The cfg in `Cargo.toml` matches `src/main.rs`.
- Published tags from this fork are `release-<git sha>` pre-releases, not `v<Cargo.toml version>`.

## Map

- Install and release: `README.md`, `.github/workflows/release.yml`, `doc/release-checklist.md`
- Open findings: `TODO.md`
- Allocator choice: [FD-3](docs/rfc/FD-3-jemalloc.md)
- Search: `src/walk.rs`, `src/cli/fd.rs`
- Tests: `tests/tests.rs`

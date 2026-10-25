# CLI parsing: lexopt only, never clap

- `lexopt` is the sole CLI parser in `src/cli/`.
- Never add `clap` (or `clap_complete`, `structopt`, `argh`, `pico-args`) to `Cargo.toml`/`Cargo.lock` or code.
- Keep the single schema in `src/cli/command.rs` as the source of truth for spellings, help, and usage.

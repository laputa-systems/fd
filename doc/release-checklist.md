# Release

How to publish binaries from this checkout. The workflow is `.github/workflows/release.yml`.

## Ship

1. Run `cargo test --locked` on the commit you want to ship.
2. Dispatch the `release` workflow on that commit. Pushing a `vX.Y.Z` tag does not start it.
3. The workflow builds three targets with `cargo build --locked --profile dist --bin fd`:
   - `aarch64-apple-darwin` on `macos-26`
   - `x86_64-unknown-linux-musl` on `ubuntu-latest`
   - `aarch64-unknown-linux-musl` on `ubuntu-24.04-arm`
4. Linux builds set the linker to `musl-gcc` and pass `-C target-feature=+crt-static -C link-arg=-fuse-ld=mold -C link-arg=-static -C link-arg=-Wl,--no-dynamic-linker`.
5. The release job requires one x86_64 ELF and one aarch64 ELF. It fails if either has an `INTERP` program header or a `NEEDED` entry. Non-ELF files are skipped.
6. It zips each binary and publishes a pre-release named `release-<github.sha>` on this repository, pointed at that commit.

There is no `.deb` and no source tarball in that release. Each zip contains one binary.

## Version string

`--version` prints the `version` field from `Cargo.toml` and the git SHA from `build.rs`. The release tag does not include the Cargo version. Change `version` in `Cargo.toml` only when the printed version should change, and rename the "Upcoming release" heading in `CHANGELOG.md` to match. The workflow does not do this.

`scripts/version-bump.sh` creates a git branch and rewrites files. Do not run it as part of this release.

## Not this release

`make archive`, `scripts/create-archive.sh`, `scripts/create-deb.sh`, and `Cross.toml` are the upstream packaging path. `make archive` copies the hand-written files in `contrib/completion/` and does not run `--gen-completions`. `scripts/create-deb.sh` still installs `autocomplete/fd.bash` and `autocomplete/fd.fish`, which this tree does not generate. Do not ship this fork with them.

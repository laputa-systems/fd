Prefix: FD
Next: 5

### FD-2 · P2 · `--version` keeps a stale git SHA after a rebuild

- **Area:** Build
- **Where:** `build.rs:40`, `src/cli/version.rs:5`
- **Problem:** `build.rs` tells Cargo to rerun only when `$GIT_DIR/HEAD` changes. On this branch that file is the one line `ref: refs/heads/master`, so a new commit updates `.git/refs/heads/master` and not the text of `HEAD`. Cargo then reuses the old `FD_GIT_SHA`. Proved on 2026-10-27: `cargo test --locked --offline` relinked `target/debug/fd` (mtime 13:54:57) and `target/debug/fd --version` printed `fd 10.5.0 (git 763b512c35b0)` while `git rev-parse --short=12 HEAD` was `57a6b17b354d`. `.git/HEAD` was last modified 2026-10-25. `.git/refs/heads/master` was last modified 2026-10-27. A clean CI checkout still embeds the right SHA, because the build script runs once with no cached env.
- **Recommendation:** Also emit `cargo:rerun-if-changed` for the ref `HEAD` points at (`git symbolic-ref -q HEAD`, then that path under the git dir). Keep failing the build when git is missing and `FD_GIT_SHA_OVERRIDE` is unset.
- **Fix with:** none
- **Effort:** S

### FD-1 · P2 · `make install` runs a removed completions flag

- **Area:** Build
- **Where:** `Makefile:23`, `Makefile:56`, `src/cli/command.rs:839`
- **Problem:** `make completions`, and therefore `make install` and `make archive`, run `fd --gen-completions`. The lexopt parser has no such option. `target/debug/fd --gen-completions bash` exits 1 with `unexpected argument '--gen-completions'`. Hand-written completion files in `contrib/completion/` are copied only after those generated files succeed, so they are never installed. The release workflow does not use this Makefile.
- **Recommendation:** Delete the bash, fish, and PowerShell recipes that call `--gen-completions`. Install the files in `contrib/completion/` directly, or drop completion installation from `make install`.
- **Fix with:** none
- **Effort:** S

### FD-3 · P3 · default and release builds omit jemalloc

- **Area:** Performance
- **Where:** `Cargo.toml:97`, `.github/workflows/release.yml:61`, commit `a7302bd`
- **Problem:** Upstream sets `default = ["use-jemalloc", "completions"]`. Commit `a7302bd` had to drop `completions` because it depended on `clap_complete`, and it set `default = []`, which also dropped `use-jemalloc`. The release workflow runs `cargo build --locked --profile dist` with no feature flag. Linux and other non-macOS builds that used to link jemalloc now use the system allocator. macOS was already excluded. The speed difference was not measured. Whether jemalloc still links under the musl `-static` flags is unproven.
- **Recommendation:** Restore `use-jemalloc` for default non-musl builds. Keep it off the musl release until a dist-profile link passes the ELF check already in `release.yml`.
- **Fix with:** none
- **Design:** [docs/rfc/FD-3-jemalloc.md](docs/rfc/FD-3-jemalloc.md)
- **Effort:** M

### FD-4 · P3 · Cargo metadata still names sharkdp's release layout

- **Area:** Build
- **Where:** `Cargo.toml:6`, `Cargo.toml:18`, `Cargo.toml:102`
- **Problem:** `homepage` and `repository` are `https://github.com/sharkdp/fd`. `package.metadata.binstall` downloads `fd-v{version}-{target}.tgz` from that GitHub repo. This fork's workflow publishes `fd-<target>.zip` on a `release-<sha>` pre-release of `laputa-systems/fd` and does not publish the crate. `cargo binstall`, or a packager following the manifest, fetches upstream artifacts, or fails, instead of this fork's binaries. The printed version is still `10.5.0`, which is the upstream crate version, so publishing this tree to crates.io as `fd-find` would collide with sharkdp's release.
- **Recommendation:** Point `repository` at `https://github.com/laputa-systems/fd`. Remove the binstall metadata until a URL matches `release.yml`. Do not `cargo publish` this tree as `fd-find` 10.5.0.
- **Fix with:** none
- **Effort:** S

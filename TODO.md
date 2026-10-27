Prefix: FD
Next: 5

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

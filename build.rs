//! Build script that embeds a short git SHA for `--version`.
//!
//! Policy:
//! - The SHA is emitted as the `FD_GIT_SHA` Cargo env var (`env!("FD_GIT_SHA")`).
//! - It must be hexadecimal (lowercase), rejecting `unknown`/`dirty`.
//! - A non-empty `FD_GIT_SHA_OVERRIDE` env var is honored for release archives
//!   without a git checkout (e.g. crates.io tarballs). Documented here because
//!   there is no `.git` in that case.
//! - Reruns on git changes using `git rev-parse --git-dir` (`.git` is a file in
//!   worktrees, so we resolve the real git dir instead of watching `.git`).
//!
//! No-git-checkout policy: if git is unavailable and no override is provided,
//! the build fails with a clear message telling the packager to set
//! `FD_GIT_SHA_OVERRIDE` to the archived commit SHA.

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn is_hex(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_hexdigit())
}

fn main() {
    // Explicit override for release archives without `.git`.
    if let Ok(ov) = env::var("FD_GIT_SHA_OVERRIDE") {
        if !ov.trim().is_empty() {
            let sha = ov.trim().to_lowercase();
            assert!(
                is_hex(&sha),
                "FD_GIT_SHA_OVERRIDE must be hexadecimal, got '{ov}'"
            );
            assert!(
                sha != "unknown" && sha != "dirty",
                "FD_GIT_SHA_OVERRIDE must not be 'unknown'/'dirty'"
            );
            println!("cargo:rerun-if-env-changed=FD_GIT_SHA_OVERRIDE");
            println!("cargo:rustc-env=FD_GIT_SHA={sha}");
            return;
        }
    }

    // Rerun on git changes. `git rev-parse --git-dir` handles worktrees where
    // `.git` is a file pointing at the real git dir.
    let git_dir = Command::new("git")
        .args(["rev-parse", "--git-dir"])
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                Some(String::from_utf8_lossy(&o.stdout).trim().to_string())
            } else {
                None
            }
        });

    if let Some(dir) = git_dir {
        let git_dir_path = PathBuf::from(dir);
        // Watch HEAD and ref updates so the SHA refreshes on commit/checkout.
        println!(
            "cargo:rerun-if-changed={}/HEAD",
            git_dir_path.display()
        );
        // `git rev-parse --git-dir` may be relative; cargo handles it relative to
        // the package root, which is sufficient to trigger reruns.
        println!("cargo:rerun-if-env-changed=FD_GIT_SHA_OVERRIDE");
    } else {
        println!("cargo:rerun-if-env-changed=FD_GIT_SHA_OVERRIDE");
    }

    let sha_out = Command::new("git")
        .args(["rev-parse", "--short=12", "HEAD"])
        .output();

    let sha = match sha_out {
        Ok(o) if o.status.success() => String::from_utf8_lossy(&o.stdout).trim().to_string(),
        _ => panic!(
            "could not determine git SHA (no .git checkout?). \
             Set FD_GIT_SHA_OVERRIDE to the archived commit SHA for release builds."
        ),
    };

    let sha = sha.to_lowercase();
    assert!(!sha.is_empty(), "empty git SHA");
    assert!(
        is_hex(&sha),
        "git SHA must be hexadecimal, got '{sha}'"
    );
    assert!(
        sha != "unknown" && sha != "dirty",
        "git SHA must not be 'unknown'/'dirty'"
    );

    println!("cargo:rustc-env=FD_GIT_SHA={sha}");
}

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn is_hex(s: &str) -> bool {
    !s.is_empty() && s.chars().all(|c| c.is_ascii_hexdigit())
}

fn main() {
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
        println!(
            "cargo:rerun-if-changed={}/HEAD",
            git_dir_path.display()
        );
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

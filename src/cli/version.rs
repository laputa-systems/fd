//! Version string with embedded git SHA. Never runs git at runtime.

pub fn version_string() -> String {
    let version = env!("CARGO_PKG_VERSION");
    let sha = env!("FD_GIT_SHA");
    format!("fd {version} (git {sha})")
}

#[cfg(test)]
mod tests {
    use super::version_string;

    #[test]
    fn version_contains_sha() {
        let v = version_string();
        assert!(v.starts_with("fd "), "version must start with 'fd ': {v}");
        assert!(v.contains("(git "), "version must embed git SHA: {v}");
        let sha = env!("FD_GIT_SHA");
        assert!(
            !sha.is_empty() && sha.chars().all(|c| c.is_ascii_hexdigit()),
            "FD_GIT_SHA must be hexadecimal, got '{sha}'"
        );
        assert!(
            sha != "unknown" && sha != "dirty",
            "FD_GIT_SHA must not be unknown/dirty"
        );
    }
}

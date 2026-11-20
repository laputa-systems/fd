use std::path::{Path, PathBuf};

pub fn git_rerun_paths(git_dir: &Path, symbolic_ref: Option<&str>) -> Vec<PathBuf> {
    let mut paths = vec![git_dir.join("HEAD")];
    let common_dir = match std::fs::read_to_string(git_dir.join("commondir")) {
        Ok(path) => git_dir.join(path.trim()).canonicalize().expect("resolve Git common directory"),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => git_dir.to_path_buf(),
        Err(error) => panic!("read Git common directory: {error}"),
    };
    if let Some(reference) = symbolic_ref
        .map(str::trim)
        .filter(|reference| !reference.is_empty())
    {
        paths.push(common_dir.join(reference));
    }
    paths.push(common_dir.join("packed-refs"));
    paths
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::git_rerun_paths;

    #[test]
    fn branch_watches_head_and_the_ref_it_points_at() {
        let git_dir = Path::new("fictional-git-dir");
        let paths = git_rerun_paths(git_dir, Some("refs/heads/topic\n"));
        assert_eq!(
            paths,
            vec![
                git_dir.join("HEAD"),
                git_dir.join("refs/heads/topic"),
                git_dir.join("packed-refs"),
            ]
        );
    }

    #[test]
    fn detached_head_watches_only_head() {
        let git_dir = Path::new("/repo/.git");
        let paths = git_rerun_paths(git_dir, None);
        assert_eq!(paths, vec![git_dir.join("HEAD"), git_dir.join("packed-refs")]);
    }

    #[test]
    fn blank_symbolic_ref_watches_only_head() {
        let git_dir = Path::new("fictional-git-dir");
        let paths = git_rerun_paths(git_dir, Some(" \n"));
        assert_eq!(paths, vec![git_dir.join("HEAD"), git_dir.join("packed-refs")]);
    }

    #[test]
    fn linked_worktree_watches_common_branch_ref() {
        let temp = tempfile::tempdir().unwrap();
        let git_dir = temp.path().join("worktrees/topic");
        std::fs::create_dir_all(&git_dir).unwrap();
        std::fs::write(git_dir.join("commondir"), "../..\n").unwrap();
        let paths = git_rerun_paths(&git_dir, Some("refs/heads/topic"));
        assert!(paths.contains(&temp.path().join("refs/heads/topic")));
    }
}

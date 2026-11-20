use std::path::{Path, PathBuf};

pub fn git_rerun_paths(git_dir: &Path, symbolic_ref: Option<&str>) -> Vec<PathBuf> {
    let mut paths = vec![git_dir.join("HEAD")];
    let common_dir = match std::fs::read_to_string(git_dir.join("commondir")) {
        Ok(path) => git_dir.join(path.trim()).canonicalize().unwrap_or_else(|error| {
            panic!("resolve Git common directory: {error}")
        }),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => git_dir.to_path_buf(),
        Err(error) => panic!("read Git common directory: {error}"),
    };
    if let Some(reference) = symbolic_ref
        .map(str::trim)
        .filter(|reference| !reference.is_empty())
    {
        paths.push(existing_path_or_parent(common_dir.join(reference)));
    }
    paths.push(existing_path_or_parent(common_dir.join("packed-refs")));
    paths
}

fn existing_path_or_parent(mut path: PathBuf) -> PathBuf {
    while !path.exists() {
        assert!(path.pop(), "Git watch path has no existing ancestor");
    }
    path
}

#[cfg(test)]
mod tests {
    use super::git_rerun_paths;

    #[test]
    fn branch_watches_head_and_the_ref_it_points_at() -> std::io::Result<()> {
        let temp = tempfile::tempdir()?;
        let git_dir = temp.path();
        std::fs::create_dir_all(git_dir.join("refs/heads"))?;
        std::fs::write(git_dir.join("refs/heads/topic"), "1234\n")?;
        std::fs::write(git_dir.join("packed-refs"), "")?;
        let paths = git_rerun_paths(git_dir, Some("refs/heads/topic\n"));
        assert_eq!(
            paths,
            vec![
                git_dir.join("HEAD"),
                git_dir.join("refs/heads/topic"),
                git_dir.join("packed-refs"),
            ]
        );
        Ok(())
    }

    #[test]
    fn detached_head_watches_head_and_common_directory() -> std::io::Result<()> {
        let temp = tempfile::tempdir()?;
        let git_dir = temp.path();
        let paths = git_rerun_paths(git_dir, None);
        assert_eq!(paths, vec![git_dir.join("HEAD"), git_dir.to_path_buf()]);
        Ok(())
    }

    #[test]
    fn blank_symbolic_ref_watches_head_and_common_directory() -> std::io::Result<()> {
        let temp = tempfile::tempdir()?;
        let git_dir = temp.path();
        let paths = git_rerun_paths(git_dir, Some(" \n"));
        assert_eq!(paths, vec![git_dir.join("HEAD"), git_dir.to_path_buf()]);
        Ok(())
    }

    #[test]
    fn linked_worktree_watches_common_branch_ref() -> std::io::Result<()> {
        let temp = tempfile::tempdir()?;
        let git_dir = temp.path().join("worktrees/topic");
        std::fs::create_dir_all(&git_dir)?;
        std::fs::write(git_dir.join("commondir"), "../..\n")?;
        std::fs::create_dir_all(temp.path().join("refs/heads"))?;
        std::fs::write(temp.path().join("refs/heads/topic"), "1234\n")?;
        let paths = git_rerun_paths(&git_dir, Some("refs/heads/topic"));
        assert!(paths.contains(&temp.path().join("refs/heads/topic")));
        Ok(())
    }

    #[test]
    fn absent_packed_refs_do_not_create_a_missing_watch() -> std::io::Result<()> {
        let temp = tempfile::tempdir()?;
        let git_dir = temp.path();
        std::fs::create_dir_all(git_dir.join("refs/heads"))?;
        std::fs::write(git_dir.join("HEAD"), "ref: refs/heads/topic\n")?;
        std::fs::write(git_dir.join("refs/heads/topic"), "1234\n")?;
        assert!(git_rerun_paths(git_dir, Some("refs/heads/topic"))
            .iter()
            .all(|path| path.exists()));
        Ok(())
    }

    #[test]
    fn packed_branch_watches_nearest_existing_ref_directory() -> std::io::Result<()> {
        let temp = tempfile::tempdir()?;
        let git_dir = temp.path();
        std::fs::create_dir_all(git_dir.join("refs/heads"))?;
        std::fs::write(git_dir.join("packed-refs"), "packed\n")?;
        let paths = git_rerun_paths(git_dir, Some("refs/heads/nested/topic"));
        assert!(paths.contains(&git_dir.join("refs/heads")));
        assert!(paths.contains(&git_dir.join("packed-refs")));
        Ok(())
    }
}

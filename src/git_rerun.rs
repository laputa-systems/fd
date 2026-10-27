use std::path::{Path, PathBuf};

pub fn git_rerun_paths(git_dir: &Path, symbolic_ref: Option<&str>) -> Vec<PathBuf> {
    let mut paths = vec![git_dir.join("HEAD")];
    if let Some(reference) = symbolic_ref
        .map(str::trim)
        .filter(|reference| !reference.is_empty())
    {
        paths.push(git_dir.join(reference));
    }
    paths
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::git_rerun_paths;

    #[test]
    fn branch_watches_head_and_the_ref_it_points_at() {
        let git_dir = Path::new(".git");
        let paths = git_rerun_paths(git_dir, Some("refs/heads/topic\n"));
        assert_eq!(
            paths,
            vec![
                git_dir.join("HEAD"),
                git_dir.join("refs/heads/topic"),
            ]
        );
    }

    #[test]
    fn detached_head_watches_only_head() {
        let git_dir = Path::new("/repo/.git");
        let paths = git_rerun_paths(git_dir, None);
        assert_eq!(paths, vec![git_dir.join("HEAD")]);
    }

    #[test]
    fn blank_symbolic_ref_watches_only_head() {
        let git_dir = Path::new(".git");
        let paths = git_rerun_paths(git_dir, Some(" \n"));
        assert_eq!(paths, vec![git_dir.join("HEAD")]);
    }
}

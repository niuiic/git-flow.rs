use super::*;

#[test]
fn has_git_t() {
    assert_eq!(Git::has_git(), true);
}

#[test]
fn in_git_project_t() {
    assert_eq!(Git::in_git_project(), true);
}

#[test]
fn switch_t() {
    let result = Git::switch("undefined");
    assert_eq!(result.is_ok(), false);
    let result = Git::switch("main");
    assert_eq!(result.is_ok(), true);
}

#[test]
fn merge_t() {
    let result = Git::merge("undefined");
    assert_eq!(result.is_ok(), false);
}

#[test]
fn rebase_t() {
    let result = Git::rebase("undefined");
    assert_eq!(result.is_ok(), false);
}

#[test]
fn cherry_pick_t() {
    let result = Git::cherry_pick(vec!["undefined".to_string()]);
    assert_eq!(result.is_ok(), false);
}

#[test]
fn del_branch_t() {
    let result = Git::del_branch("undefined");
    assert_eq!(result.is_ok(), false);
}

#[test]
fn diff_commits_t() {
    let result = Git::diff_commits("main", "test");
    assert_eq!(result.is_ok(), false)
}

#[test]
fn create_branch_t() {
    let result = Git::create_branch("main", "main");
    assert_eq!(result.is_ok(), false)
}

#[test]
fn get_branches_t() {
    let result = Git::get_branches().unwrap();
    assert_eq!(result.iter().find(|x| x.as_str() == "main").is_some(), true);
}

#[test]
fn diff_logs() {
    Git::diff_logs("main", "main").unwrap();
}

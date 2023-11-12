use super::*;
use crate::config::read::read_config;

#[test]
fn name() {
    let config_list = read_config().unwrap();
    let (branch_type, branch_name) =
        get_branch_type_name(&config_list, &vec!["feat/test".to_string()]).unwrap();
    assert_eq!(branch_type, "feature");
    assert_eq!(branch_name, "test");
    let (branch_type, branch_name) = get_branch_type_name(
        &config_list,
        &vec!["feature".to_string(), "test".to_string()],
    )
    .unwrap();
    assert_eq!(branch_type, "feature");
    assert_eq!(branch_name, "test");
}

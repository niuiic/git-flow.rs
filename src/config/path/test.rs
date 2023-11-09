use std::str::FromStr;

use regex::Regex;

use super::*;

#[test]
fn get_global_config_path_t() {
    if env::consts::OS == "linux" {
        let global_config_path = get_global_config_path().unwrap();
        let regex = Regex::new(r"^/home/.*/.config/git-flow/.git-flow.json$").unwrap();
        assert_eq!(regex.is_match(&global_config_path), true);
    }
}

#[test]
fn get_local_config_path_t() {
    let local_config_path = get_local_config_path().unwrap();
    let mut path = PathBuf::from_str(&local_config_path).unwrap();
    path.pop();
    assert_eq!(path.join(".git").is_dir(), true);
}

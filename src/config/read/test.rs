use super::*;

#[test]
fn read_config_t() {
    assert_eq!(read_config().is_ok(), true);
}

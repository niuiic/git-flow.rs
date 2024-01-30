use super::*;

#[test]
fn read_config_t() {
    assert_eq!(read_config(None).is_ok(), true);
}

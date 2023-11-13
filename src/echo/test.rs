use super::*;

#[test]
fn echo_t() {
    Echo::warn("hello");
    Echo::info("hello");
    Echo::success("hello");
    Echo::error("hello");
    Echo::progress("hello");
}

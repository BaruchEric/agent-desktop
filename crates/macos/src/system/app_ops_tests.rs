use super::*;

#[test]
fn open_app_args_preserve_current_focus() {
    assert_eq!(open_app_args("Mail"), ["-g", "-a", "Mail"]);
}

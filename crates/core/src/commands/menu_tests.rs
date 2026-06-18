use super::parse_path;

#[test]
fn splits_and_trims_segments() {
    assert_eq!(
        parse_path("Format > Make Plain Text"),
        vec!["Format".to_string(), "Make Plain Text".to_string()]
    );
}

#[test]
fn single_segment_is_kept() {
    assert_eq!(parse_path("File"), vec!["File".to_string()]);
}

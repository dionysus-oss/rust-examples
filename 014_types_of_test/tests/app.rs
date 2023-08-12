use app::run_with_strings;

#[test]
fn test_run() {
    let count = run_with_strings(["of".to_string(), "professor".to_string()].into_iter());
    assert_eq!(1, count);
}

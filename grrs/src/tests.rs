use super::answer;
use super::find_matches;

#[test]
fn check_answer_validity() {
    assert_eq!(answer(), 42);
}

#[test]
fn test_find_matches() {
    let result = find_matches("lorem ipsum\ndolor sit amet", "lorem");
    assert_eq!(result, vec!["lorem ipsum"]);
}

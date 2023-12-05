pub fn chunks_of_non_empty_lines(text: &str) -> Vec<Vec<&str>> {
    let mut output: Vec<Vec<&str>> = Vec::with_capacity(text.len());
    let mut was_in_empty = true;

    for next_line in text.lines() {
        let trimmed = next_line.trim();
        if trimmed.is_empty() {
            was_in_empty = true;
        } else {
            if was_in_empty {
                output.push(Vec::new());
            }
            was_in_empty = false;
            output.last_mut().unwrap().push(next_line);
        }
    }

    output
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn parsing_chunks_of_non_empty_lines() {
        assert_case("", &vec![]);
        assert_case("aaa\naa\n", &vec![vec!["aaa", "aa"]]);
        assert_case(
            "  \naaa\naa\n\n  \nbb\n\n",
            &vec![vec!["aaa", "aa"], vec!["bb"]],
        );
        fn assert_case(input: &str, expected: &[Vec<&str>]) {
            let actual = chunks_of_non_empty_lines(input);
            assert_eq!(expected, &actual, "Input: {}", input);
        }
    }
}

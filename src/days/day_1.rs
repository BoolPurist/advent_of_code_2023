use once_cell::sync::Lazy;

pub fn handle(input: String) -> String {
    let parsed = parse(&input);
    parsed.into_iter().sum::<u32>().to_string()
}

fn parse(input: &str) -> Vec<u32> {
    input.lines().map(parse_line_task_1).collect()
}
pub fn handle_task_2(input: String) -> String {
    input
        .lines()
        .enumerate()
        .map(|(index, value)| {
            let value = parse_line_task_2(value);
            value.unwrap_or_else(|| panic!("No 2 digits at line {}", index + 1))
        })
        .sum::<u32>()
        .to_string()
}

fn parse_line_task_1(input: &str) -> u32 {
    let iter = input
        .trim()
        .chars()
        .filter_map(|may_digit| may_digit.to_digit(10));
    calc_numbers(iter).unwrap()
}

fn calc_numbers(iterator: impl IntoIterator<Item = u32>) -> Option<u32> {
    let mut iter = iterator.into_iter();
    match (iter.next(), iter.last()) {
        (None, None) => None,
        (None, Some(_)) => unreachable!(),
        (Some(only_digit), None) => Some(only_digit * 10 + only_digit),
        (Some(left), Some(right)) => Some(left * 10 + right),
    }
}

fn parse_line_task_2(input: &str) -> Option<u32> {
    let start: u32 = iterate_from(input.chars(), |buffer, next| buffer.push(next));
    let last: u32 = iterate_from(input.chars().rev(), |buffer, next| {
        *buffer = next.to_string() + &buffer;
    });
    return Some(last + start * 10);
    fn iterate_from(
        input: impl Iterator<Item = char>,
        on_buffer_change: impl Fn(&mut String, char),
    ) -> u32 {
        let mut buffer = String::default();

        for next in input {
            on_buffer_change(&mut buffer, next);
            match CHARS_TO_NUMBER
                .iter()
                .find(|(word, _)| buffer.contains(word.as_ref()))
            {
                Some((_, index)) => {
                    return *index;
                }
                None => (),
            }
        }
        0
    }
}

static CHARS_TO_NUMBER: Lazy<Vec<(Box<str>, u32)>> = Lazy::new(|| {
    let words = [
        Box::from("one"),
        Box::from("two"),
        Box::from("three"),
        Box::from("four"),
        Box::from("five"),
        Box::from("six"),
        Box::from("seven"),
        Box::from("eight"),
        Box::from("nine"),
    ]
    .into_iter()
    .enumerate()
    .map(|(index, word)| (word, (index + 1) as u32));
    let numbers = (1..=9).map(|number| (Box::from(number.to_string().as_str()), number));
    words.chain(numbers).collect()
});

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn parsing_day_1_line_task_2() {
        assert_case("two1nine", 29);
        assert_case("2s", 22);
        assert_case("one", 11);
        assert_case("two", 22);
        assert_case("eightwo", 82);
        assert_case("three", 33);
        assert_case("four", 44);
        assert_case("afivec", 55);
        assert_case("sisixsi", 66);
        assert_case("sesevenx", 77);
        assert_case("eightx", 88);
        assert_case("ninenin", 99);
        assert_case("eightwothree", 83);
        assert_case("abcone2threexyz", 13);
        assert_case("xtwone3four", 24);
        assert_case("4nineeightseven2", 42);
        assert_case("4xxnine", 49);
        assert_case("4xxnine4", 44);
        assert_case("ninenine", 99);
        assert_case("d6", 66);
        assert_case("9mpjm", 99);
        assert_case("fkfzrdjvmnv9onemhlsjzrmxzzjfourjkvvgn", 94);
        assert_case("ninexxnin4", 94);
        assert_case("zoneight234", 14);
        assert_case("7pqrstsixteen", 76);
        assert_case("9mpjm", 99);

        fn assert_case(input: &str, expected: u32) {
            let actual = parse_line_task_2(input);
            assert_eq!(
                expected,
                actual.unwrap(),
                "Expected: {:?}, Actual: {:?}, input: {}",
                expected,
                actual,
                input
            );
        }
    }
    #[test]
    fn parsing_day_1() {
        assert_case(
            "1abc2
 pqr3stu8vwx",
            vec![12, 38],
        );
        assert_case(
            "1abc2
 pqr3stu8vwx
 a1b2c3d4e5f
 treb7uchet",
            vec![12, 38, 15, 77],
        );
        fn assert_case(input: &str, expected: Vec<u32>) {
            let actual = parse(input);
            assert_eq!(
                expected, actual,
                "Expected: {:?}, Actual: {:?}",
                expected, actual
            );
        }
    }
    #[test]
    fn parsing_line_day_1() {
        assert_case("1abc2", 12);
        assert_case("a1b2c3d4e5f", 15);
        assert_case("treb7uchet", 77);
        fn assert_case(input: &str, expected: u32) {
            let actual = parse_line_task_1(input);
            assert_eq!(
                expected, actual,
                "Expected: {:?}, Actual: {:?}",
                expected, actual
            );
        }
    }
}

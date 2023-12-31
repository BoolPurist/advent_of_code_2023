use std::rc::Rc;

pub fn handle_task(input: String) -> String {
    let parsed = parse(&input);
    parsed
        .iter()
        .map(get_seqs)
        .map(|seqs| calculate_score_of(&seqs))
        .sum::<u32>()
        .to_string()
}
pub fn handle_task_2(input: String) -> String {
    let parsed = parse(&input);
    let mut plot: Vec<(usize, Vec<usize>)> = parsed
        .iter()
        .map(get_seqs)
        .enumerate()
        .map(|(index, seqs)| (1, next_seq(index, &seqs)))
        .collect();
    plot.first_mut().unwrap().0 = 1;

    for index in 0..plot.len() {
        let (count, seq) = plot.get(index).unwrap().clone();

        for &next_card in seq.iter() {
            plot.get_mut(next_card).unwrap().0 += count;
        }
    }

    let sum = plot.into_iter().fold(0usize, |acc, (count, _)| acc + count);

    return sum.to_string();
    fn next_seq(index: usize, seqs: &[u32]) -> Vec<usize> {
        let start = index + 1;
        let end = start + seqs.len();
        (start..end).collect()
    }
}
fn get_seqs(cards: &Card) -> Rc<[u32]> {
    cards
        .actual
        .iter()
        .filter(|&next| cards.winning.contains(next))
        .cloned()
        .collect()
}

fn calculate_score_of(points: &[u32]) -> u32 {
    if points.is_empty() {
        return 0;
    }
    let mut val = 1;
    for _ in 1..points.len() {
        val *= 2;
    }
    val
}

fn parse(input: &str) -> Vec<Card> {
    input
        .lines()
        .map(|line| {
            let mut numbers = line.trim().split(":").skip(1).next().unwrap().split("|");

            let (winning, actual) = (numbers.next().unwrap(), numbers.next().unwrap());
            let winning = split_numbers(winning);
            let actual = split_numbers(actual);
            return Card { winning, actual };
            fn split_numbers(input: &str) -> Vec<u32> {
                input
                    .split_whitespace()
                    .map(|number| number.parse::<u32>().unwrap())
                    .collect()
            }
        })
        .collect()
}

#[derive(Debug)]
struct Card {
    winning: Vec<u32>,
    actual: Vec<u32>,
}
#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn day_4_parse() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let parsed = parse(input);
        insta::assert_debug_snapshot!(parsed);
    }
    #[test]
    fn day_4_get_seqs() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let parsed = parse(input);
        let actual = get_seqs(parsed.get(0).unwrap());
        let expected: Vec<u32> = vec![83, 86, 17, 48];
        assert_eq!(expected.as_slice(), actual.as_ref());
    }
    #[test]
    fn day_4_get_score_of() {
        assert_case(&[1, 2, 3, 4], 8);
        assert_case(&[], 0);
        assert_case(&[1, 2], 2);
        fn assert_case(input: &[u32], expected: u32) {
            let actual = calculate_score_of(input);
            assert_eq!(expected, actual, "Input: {:?}", input);
        }
    }
}

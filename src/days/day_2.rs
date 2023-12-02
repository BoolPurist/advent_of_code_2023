pub fn handle_task(input: String) -> String {
    let parsed = parse_input(&input);
    let all_invalid = get_all_valid_games(
        &parsed,
        &Set {
            red: 12,
            blue: 14,
            green: 13,
        },
    );
    all_invalid.into_iter().sum::<u32>().to_string()
}
pub fn handle_task_2(input: String) -> String {
    let parsed = parse_input(&input);
    let maximums: Vec<Set> = parsed
        .into_iter()
        .map(|game| {
            game.sets.into_iter().fold(Set::default(), |max, next| Set {
                red: max.red.max(next.red),
                blue: max.blue.max(next.blue),
                green: max.green.max(next.green),
            })
        })
        .collect();
    maximums
        .into_iter()
        .map(|max_set| max_set.red * max_set.blue * max_set.green)
        .sum::<u32>()
        .to_string()
}
#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

impl Game {
    fn new(id: u32) -> Self {
        Self {
            id,
            sets: Default::default(),
        }
    }
}

#[derive(Default, Debug)]
struct Set {
    red: u32,
    blue: u32,
    green: u32,
}

fn parse_input(input: &str) -> Vec<Game> {
    input.lines().map(parse_game_line_input).collect()
}

fn get_all_valid_games(games: &[Game], conf: &Set) -> Vec<u32> {
    games
        .into_iter()
        .filter_map(|game| {
            if is_valid_game(game, conf) {
                Some(game.id)
            } else {
                None
            }
        })
        .collect()
}

fn is_valid_game(game: &Game, conf: &Set) -> bool {
    let sets = &game.sets;
    sets.iter()
        .all(|set| set.red <= conf.red && set.blue <= conf.blue && set.green <= conf.green)
}

fn parse_game_line_input(line: &str) -> Game {
    let trimmed = line.trim();
    let mut splitted = trimmed.split(":");
    match (splitted.next(), splitted.next()) {
        (Some(game), Some(sets)) => {
            let id: u32 = game.strip_prefix("Game ").unwrap().parse().unwrap();
            let splitted = sets.split(";");
            let mut game = Game::new(id);
            for next in splitted {
                let comma_splitted = next.split(",");
                let mut set = Set::default();
                for next_comma in comma_splitted {
                    let mut space_seperated = next_comma.trim().split_whitespace();
                    match (space_seperated.next(), space_seperated.next()) {
                        (Some(number), Some(keyword)) => {
                            let number: u32 = number.parse().unwrap();
                            match keyword {
                                "red" => set.red = number,
                                "blue" => set.blue = number,
                                "green" => set.green = number,
                                _ => unimplemented!(),
                            }
                        }
                        _ => unimplemented!(),
                    }
                }
                game.sets.push(set);
            }
            game
        }
        _ => unimplemented!(),
    }
}

#[cfg(test)]
mod testing {
    use super::*;

    #[test]
    fn test_parsing() {
        let actual = parse_game_line_input(
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
        );
        insta::assert_debug_snapshot!(actual);
    }
}

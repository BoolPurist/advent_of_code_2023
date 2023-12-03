use crate::prelude::*;
use std::collections::HashSet;

pub fn handle_task(input: String) -> String {
    let parsed = parse(&input);
    let part_numbers = get_all_part_number(&parsed);
    let part_numbers_vals = calc_part_numbers(&parsed, &part_numbers);
    part_numbers_vals.into_iter().sum::<usize>().to_string()
}
pub fn handle_task_2(input: String) -> String {
    let parsed = parse(&input);
    get_part_number_gears(&parsed)
        .into_iter()
        .map(|(left, right)| calc_part_numbers(&parsed, &[left, right]))
        .map(|numbers| numbers.into_iter().product::<usize>())
        .sum::<usize>()
        .to_string()
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Symbol {
    Symb(char),
    Digit(u32),
    Nothing,
}

impl From<char> for Symbol {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Nothing,
            sym if sym.is_digit(10) => Self::Digit(sym.to_digit(10).unwrap()),
            sym => Self::Symb(sym),
        }
    }
}
fn get_part_number_gears(grid: &Array2D<Symbol>) -> Vec<(ColumnRange, ColumnRange)> {
    let mut already_found: HashSet<Coords> = HashSet::new();
    get_all_symbols(grid)
        .into_iter()
        .filter(|coords| matches!(grid_2d::access_grid(grid, *coords), Symbol::Symb('*')))
        .filter_map(|coords| {
            let might_be_two = get_part_number_around(grid, &mut already_found, coords);
            if might_be_two.len() == 2 {
                Some((
                    might_be_two.get(0).unwrap().clone(),
                    might_be_two.get(1).unwrap().clone(),
                ))
            } else {
                None
            }
        })
        .collect()
}

fn parse(input: &str) -> Array2D<Symbol> {
    grid_2d::text_to_grid(input, Symbol::from)
}

fn get_all_symbols(grid: &Array2D<Symbol>) -> Vec<Coords> {
    let mut all_coords = Vec::default();
    for coords in grid_2d::all_coords(grid) {
        let val = grid_2d::access_grid(grid, coords);
        if matches!(val, Symbol::Symb(_)) {
            all_coords.push(coords);
        }
    }
    all_coords
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct ColumnRange {
    row: usize,
    start: usize,
    end: usize,
}

fn calc_part_numbers(grid: &Array2D<Symbol>, columns: &[ColumnRange]) -> Vec<usize> {
    let mut calc_numbers = Vec::new();

    for next_range in columns {
        let mut factor = 1;
        let mut number = 0;
        for next_number in (next_range.start..=next_range.end).rev() {
            if let Symbol::Digit(numeric_val) = grid_2d::access_grid(
                grid,
                Coords {
                    x: next_number,
                    y: next_range.row,
                },
            ) {
                number += factor * (*numeric_val) as usize;
                factor *= 10;
            } else {
                unreachable!()
            }
        }
        calc_numbers.push(number);
    }
    calc_numbers
}

fn get_part_number_around(
    grid: &Array2D<Symbol>,
    already_found: &mut HashSet<Coords>,
    coords: Coords,
) -> Vec<ColumnRange> {
    let mut output = Vec::new();

    travers_and_update(
        grid,
        already_found,
        coords,
        |grid, coords| grid_2d::go_left(grid, coords),
        &mut output,
    );
    travers_and_update(
        grid,
        already_found,
        coords,
        |grid, coords| grid_2d::go_right(grid, coords),
        &mut output,
    );
    travers_and_update(
        grid,
        already_found,
        coords,
        |grid, coords| grid_2d::go_up(grid, coords),
        &mut output,
    );
    travers_and_update(
        grid,
        already_found,
        coords,
        |grid, coords| grid_2d::go_down(grid, coords),
        &mut output,
    );
    travers_and_update(
        grid,
        already_found,
        coords,
        |grid, coords| grid_2d::go_left_up(grid, coords),
        &mut output,
    );
    travers_and_update(
        grid,
        already_found,
        coords,
        |grid, coords| grid_2d::go_left_down(grid, coords),
        &mut output,
    );
    travers_and_update(
        grid,
        already_found,
        coords,
        |grid, coords| grid_2d::go_right_up(grid, coords),
        &mut output,
    );
    travers_and_update(
        grid,
        already_found,
        coords,
        |grid, coords| grid_2d::go_right_down(grid, coords),
        &mut output,
    );
    return output;
    fn travers_and_update(
        grid: &Array2D<Symbol>,
        already_found: &mut HashSet<Coords>,
        coords: Coords,
        on_start: impl Fn(&Array2D<Symbol>, Coords) -> grid_2d::MaybeNextCell<Symbol>,
        output: &mut Vec<ColumnRange>,
    ) {
        if let Some(to_add) = travers_colums_at(grid, already_found, coords, on_start) {
            output.push(to_add);
        }
    }
    fn travers_colums_at(
        grid: &Array2D<Symbol>,
        already_found: &mut HashSet<Coords>,
        coords: Coords,
        on_start: impl Fn(&Array2D<Symbol>, Coords) -> grid_2d::MaybeNextCell<Symbol>,
    ) -> Option<ColumnRange> {
        let (sym, start_coords) = on_start(grid, coords)?;
        if let Symbol::Digit(_) = sym {
            let (row, column) = (start_coords.y, start_coords.x);
            if !already_found.insert(start_coords) {
                return None;
            }
            let mut range = ColumnRange {
                row,
                start: column,
                end: column,
            };

            let mut current_coords = start_coords;
            while let Some((Symbol::Digit(_), Coords { x, .. })) =
                grid_2d::go_left(grid, current_coords)
            {
                current_coords = Coords { y: row, x };
                if !already_found.insert(current_coords) {
                    break;
                }

                range.start = x;
            }

            current_coords = start_coords;
            while let Some((Symbol::Digit(_), Coords { x, .. })) =
                grid_2d::go_right(grid, current_coords)
            {
                current_coords = Coords { y: row, x };
                if !already_found.insert(current_coords) {
                    break;
                }

                range.end = x;
            }

            Some(range)
        } else {
            None
        }
    }
}
fn get_all_part_number(grid: &Array2D<Symbol>) -> Vec<ColumnRange> {
    let mut already_found: HashSet<Coords> = HashSet::new();
    let mut output = Vec::new();
    for coords in get_all_symbols(grid) {
        let to_add = get_part_number_around(grid, &mut already_found, coords);
        output.extend(to_add.into_iter());
    }

    return output;
}

#[cfg(test)]
mod testing {
    use super::*;

    const TEST_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    #[test]
    fn parsing_day_3_input_all_symbols_find() {
        let parsed = parse(TEST_INPUT);
        let mut all_sym_coords = get_all_symbols(&parsed);
        let mut expected: Vec<Coords> = vec![
            (1, 3).into(),
            (3, 6).into(),
            (4, 3).into(),
            (5, 5).into(),
            (8, 3).into(),
            (8, 5).into(),
        ];
        expected.sort();
        all_sym_coords.sort();
        assert_eq!(&expected, &all_sym_coords);
    }
    const COLUMS_RANGES_INPUT: &[ColumnRange] = &[
        ColumnRange {
            row: 0,
            start: 0,
            end: 2,
        },
        ColumnRange {
            row: 2,
            start: 2,
            end: 3,
        },
        ColumnRange {
            row: 2,
            start: 6,
            end: 8,
        },
        ColumnRange {
            row: 4,
            start: 0,
            end: 2,
        },
        ColumnRange {
            row: 6,
            start: 2,
            end: 4,
        },
        ColumnRange {
            row: 7,
            start: 6,
            end: 8,
        },
        ColumnRange {
            row: 9,
            start: 1,
            end: 3,
        },
        ColumnRange {
            row: 9,
            start: 5,
            end: 7,
        },
    ];
    #[test]
    fn parsing_day_3_all_part_number() {
        let parsed = parse(TEST_INPUT);
        let mut part_numbers = get_all_part_number(&parsed);
        let mut expected: Vec<ColumnRange> = COLUMS_RANGES_INPUT.to_vec();
        part_numbers.sort();
        expected.sort();

        assert_eq!(&expected, &part_numbers);
    }
    #[test]
    fn day_3_clac_numbers_from_part_number() {
        let parsed = parse(TEST_INPUT);
        let mut actual = calc_part_numbers(&parsed, COLUMS_RANGES_INPUT);

        let mut expected: Vec<usize> = vec![35, 467, 633, 617, 592, 755, 664, 598];
        expected.sort();
        actual.sort();

        assert_eq!(&expected, &actual);
    }
    #[test]
    fn day_3_get_gear_part_number() {
        let parsed = parse(TEST_INPUT);

        let mut actual = get_part_number_gears(&parsed);

        let mut expected: Vec<(ColumnRange, ColumnRange)> = vec![
            (
                ColumnRange {
                    row: 2,
                    start: 2,
                    end: 3,
                },
                ColumnRange {
                    row: 0,
                    start: 0,
                    end: 2,
                },
            ),
            (
                ColumnRange {
                    row: 9,
                    start: 5,
                    end: 7,
                },
                ColumnRange {
                    row: 7,
                    start: 6,
                    end: 8,
                },
            ),
        ];
        expected.sort();
        actual.sort();

        assert_eq!(&expected, &actual);
    }
}

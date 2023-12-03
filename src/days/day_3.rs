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

#[derive(Debug, PartialEq, Eq)]
pub enum Symbol {
    Symb(char),
    Digit(u32),
    Nothing,
}

type Grid<'a> = &'a [Vec<Symbol>];

impl From<char> for Symbol {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Nothing,
            sym if sym.is_digit(10) => Self::Digit(sym.to_digit(10).unwrap()),
            sym => Self::Symb(sym),
        }
    }
}
fn get_part_number_gears(grid: Grid) -> Vec<(ColumnRange, ColumnRange)> {
    let mut already_found: HashSet<(usize, usize)> = HashSet::new();
    get_all_symbols(grid)
        .into_iter()
        .filter(|(row, column)| matches!(access_grid(grid, *row, *column), Symbol::Symb('*')))
        .filter_map(|(row, column)| {
            let might_be_two = get_part_number_around(grid, &mut already_found, row, column);
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

fn parse(input: &str) -> Vec<Vec<Symbol>> {
    input
        .lines()
        .map(|line| line.trim().chars().map(Symbol::from).collect())
        .collect()
}

fn get_all_symbols(grid: Grid) -> Vec<(usize, usize)> {
    let mut coords = Vec::default();
    for (row_i, row) in grid.into_iter().enumerate() {
        for (colum_i, colum) in row.into_iter().enumerate() {
            if matches!(colum, Symbol::Symb(_)) {
                coords.push((row_i, colum_i));
            }
        }
    }
    coords
}

fn access_grid(grid: Grid, row: usize, column: usize) -> &Symbol {
    grid.get(row).unwrap().get(column).unwrap()
}
#[derive(Debug)]
pub struct MovedToCell<'a> {
    symbol: &'a Symbol,
    row: usize,
    column: usize,
}

impl<'a> MovedToCell<'a> {
    pub fn new(grid: Grid<'a>, row: usize, column: usize) -> Self {
        Self {
            symbol: access_grid(grid, row, column),
            row,
            column,
        }
    }
}
fn go_left(grid: Grid, row: usize, column: usize) -> Option<MovedToCell> {
    match column.overflowing_sub(1) {
        (_, true) => None,
        (new_column, false) => Some(MovedToCell::new(grid, row, new_column)),
    }
}
fn go_right(grid: Grid, row: usize, column: usize) -> Option<MovedToCell> {
    let new_column = column + 1;
    if new_column < grid.get(0).unwrap().len() {
        Some(MovedToCell::new(grid, row, new_column))
    } else {
        None
    }
}
fn go_up(grid: Grid, row: usize, column: usize) -> Option<MovedToCell> {
    match row.overflowing_sub(1) {
        (_, true) => None,
        (new_row, false) => Some(MovedToCell::new(grid, new_row, column)),
    }
}
fn go_down(grid: Grid, row: usize, column: usize) -> Option<MovedToCell> {
    let new_row = row + 1;
    if new_row < grid.len() {
        Some(MovedToCell::new(grid, new_row, column))
    } else {
        None
    }
}
fn go_left_up(grid: Grid, row: usize, column: usize) -> Option<MovedToCell> {
    let moved = go_left(grid, row, column)?;
    go_up(grid, moved.row, moved.column)
}
fn go_right_up(grid: Grid, row: usize, column: usize) -> Option<MovedToCell> {
    let moved = go_right(grid, row, column)?;
    go_up(grid, moved.row, moved.column)
}
fn go_left_down(grid: Grid, row: usize, column: usize) -> Option<MovedToCell> {
    let moved = go_left(grid, row, column)?;
    go_down(grid, moved.row, moved.column)
}
fn go_right_down(grid: Grid, row: usize, column: usize) -> Option<MovedToCell> {
    let moved = go_right(grid, row, column)?;
    go_down(grid, moved.row, moved.column)
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct ColumnRange {
    row: usize,
    start: usize,
    end: usize,
}

fn calc_part_numbers(grid: Grid, columns: &[ColumnRange]) -> Vec<usize> {
    let mut calc_numbers = Vec::new();
    for next_range in columns {
        let mut factor = 1;
        let mut number = 0;
        for next_number in (next_range.start..=next_range.end).rev() {
            if let Symbol::Digit(numeric_val) = access_grid(grid, next_range.row, next_number) {
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
    grid: Grid,
    already_found: &mut HashSet<(usize, usize)>,
    row: usize,
    colum: usize,
) -> Vec<ColumnRange> {
    let mut output = Vec::new();

    travers_and_update(
        grid,
        already_found,
        row,
        colum,
        |grid, r, c| go_left(grid, r, c),
        &mut output,
    );
    travers_and_update(
        grid,
        already_found,
        row,
        colum,
        |grid, r, c| go_right(grid, r, c),
        &mut output,
    );
    travers_and_update(
        grid,
        already_found,
        row,
        colum,
        |grid, r, c| go_up(grid, r, c),
        &mut output,
    );
    travers_and_update(
        grid,
        already_found,
        row,
        colum,
        |grid, r, c| go_down(grid, r, c),
        &mut output,
    );
    travers_and_update(
        grid,
        already_found,
        row,
        colum,
        |grid, r, c| go_left_up(grid, r, c),
        &mut output,
    );
    travers_and_update(
        grid,
        already_found,
        row,
        colum,
        |grid, r, c| go_left_down(grid, r, c),
        &mut output,
    );
    travers_and_update(
        grid,
        already_found,
        row,
        colum,
        |grid, r, c| go_right_up(grid, r, c),
        &mut output,
    );
    travers_and_update(
        grid,
        already_found,
        row,
        colum,
        |grid, r, c| go_right_down(grid, r, c),
        &mut output,
    );
    return output;
    fn travers_and_update(
        grid: Grid,
        already_found: &mut HashSet<(usize, usize)>,
        row: usize,
        column: usize,
        on_start: impl Fn(Grid, usize, usize) -> Option<MovedToCell>,
        output: &mut Vec<ColumnRange>,
    ) {
        if let Some(to_add) = travers_colums_at(grid, already_found, row, column, on_start) {
            output.push(to_add);
        }
    }
    fn travers_colums_at(
        grid: Grid,
        already_found: &mut HashSet<(usize, usize)>,
        row: usize,
        column: usize,
        on_start: impl Fn(Grid, usize, usize) -> Option<MovedToCell>,
    ) -> Option<ColumnRange> {
        let start = on_start(grid, row, column)?;
        let sym = start.symbol;
        if let Symbol::Digit(_) = sym {
            let (row, column) = (start.row, start.column);
            if !already_found.insert((row, column)) {
                return None;
            }
            let mut range = ColumnRange {
                row,
                start: column,
                end: column,
            };
            let mut current_colum = column;
            while let Some(MovedToCell {
                symbol: Symbol::Digit(_),
                column,
                ..
            }) = go_left(grid, row, current_colum)
            {
                current_colum = column;
                if !already_found.insert((row, current_colum)) {
                    break;
                }

                range.start = current_colum;
            }
            let mut current_colum = column;
            while let Some(MovedToCell {
                symbol: Symbol::Digit(_),
                column,
                ..
            }) = go_right(grid, row, current_colum)
            {
                current_colum = column;
                if !already_found.insert((row, current_colum)) {
                    break;
                }
                range.end = current_colum;
            }

            Some(range)
        } else {
            None
        }
    }
}
fn get_all_part_number(grid: Grid) -> Vec<ColumnRange> {
    let mut already_found: HashSet<(usize, usize)> = HashSet::new();
    let mut output = Vec::new();
    for (row, colum) in get_all_symbols(grid) {
        let to_add = get_part_number_around(grid, &mut already_found, row, colum);
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
        let all_sym_coords = get_all_symbols(&parsed);
        let expected: Vec<(usize, usize)> = vec![(1, 3), (3, 6), (4, 3), (5, 5), (8, 3), (8, 5)];
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

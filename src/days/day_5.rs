use std::{cmp::Reverse, rc::Rc};

use crate::utils;
type ImmutableSeq<T> = Rc<[T]>;
type ImmutableNumberSeq = ImmutableSeq<NumericVal>;
type NumericVal = u64;

pub fn handle_task(input: String) -> String {
    let parsed = parsing(&input);
    let locations: ImmutableNumberSeq = parsed
        .seeds
        .clone()
        .into_iter()
        .map(|&next_seed| get_seed_to_location_path(next_seed, &parsed))
        .collect();
    let minimal_location = locations.into_iter().min().unwrap();
    minimal_location.to_string()
}
#[derive(Debug, Default, Clone, PartialEq, Eq, Copy)]
pub struct ItemRange {
    start: u64,
    end: u64,
}
pub fn handle_task_3(input: String) -> String {
    let parsed = parsing(&input);
    let (odd, even) = (parsed.seeds.clone(), parsed.seeds.clone());
    let mut previous: Vec<ItemRange> = even
        .into_iter()
        .step_by(2)
        .zip(odd.into_iter().skip(1).step_by(2))
        .map(|(&start, &range)| {
            let end = ((start) + range) - 1;
            ItemRange { start, end }
        })
        .collect();
    for next_step in [
        &parsed.seeds_to_soil,
        &parsed.soil_to_fertilizer,
        &parsed.fertilizer_to_water,
        &parsed.water_to_light,
        &parsed.light_to_temperature,
        &parsed.temperature_to_humidity,
        &parsed.humidity_to_location,
    ] {
        let merged = merge_ranges(previous);
        let mut current = Vec::new();
        let mut upper_bound = NumericVal::MIN;
        let mut lower_bound = NumericVal::MAX;
        for next_range in merged.iter() {
            for next_row in next_step.clone().into_iter() {
                upper_bound = upper_bound.max(next_row.source_end);
                lower_bound = lower_bound.min(next_row.source_start);
                let mapped = get_mapped_range_from(*next_range, next_row);
                if let Some(to_push) = mapped {
                    current.push(to_push);
                }
            }
            if upper_bound < next_range.end {
                current.push(ItemRange {
                    start: upper_bound + 1,
                    end: next_range.end,
                })
            }
            if lower_bound > next_range.start {
                current.push(ItemRange {
                    start: lower_bound.saturating_sub(1),
                    end: next_range.end,
                })
            }
        }

        previous = current.clone();
        dbg!(&previous);
    }
    let miminum_val = previous
        .into_iter()
        .fold(NumericVal::MAX, |acc, next| acc.min(next.start));

    miminum_val.to_string()
}
pub fn handle_task_2(input: String) -> String {
    let parsed = parsing(&input);
    let (odd, even) = (parsed.seeds.clone(), parsed.seeds.clone());
    let mut minum_val = NumericVal::MAX;
    even.into_iter()
        .step_by(2)
        .zip(odd.into_iter().skip(1).step_by(2))
        .for_each(|(&start, &range)| {
            let end = (start) + range;

            for next_seed in start..end {
                let location = get_seed_to_location_path(next_seed, &parsed);
                minum_val = minum_val.min(location);
            }
        });

    return format!("{:#?}", minum_val);
}

pub fn get_seed_to_location_path(seed: NumericVal, mapping: &TableMapping) -> NumericVal {
    let mut current_val = seed;
    for next_step in [
        &mapping.seeds_to_soil,
        &mapping.soil_to_fertilizer,
        &mapping.fertilizer_to_water,
        &mapping.water_to_light,
        &mapping.light_to_temperature,
        &mapping.temperature_to_humidity,
        &mapping.humidity_to_location,
    ] {
        let new_val = get_mapping(current_val, next_step);
        current_val = new_val;
    }
    current_val
}

pub fn get_mapping(source: NumericVal, rows: &ImmutableSeq<RowMapping>) -> NumericVal {
    return rows
        .into_iter()
        .find_map(|row| get_withing_range(source, row))
        .unwrap_or(source);
}

fn get_withing_range(val: NumericVal, row: &RowMapping) -> Option<NumericVal> {
    let start = row.source_start;

    if val >= start && val <= row.source_end {
        let offset = val - start;
        Some(row.dest_start + offset)
    } else {
        None
    }
}

fn merge_ranges(mut to_merge: Vec<ItemRange>) -> Vec<ItemRange> {
    to_merge.sort_by_key(|key| (key.start, Reverse(key.end)));
    let mut to_merge = to_merge.into_iter();
    if let Some(mut current_item) = to_merge.next() {
        let mut merged: Vec<ItemRange> = Vec::with_capacity(to_merge.len());
        for next in to_merge {
            let bound = current_item.end + 1;
            if bound >= next.start {
                current_item = ItemRange {
                    start: next.start.min(current_item.start),
                    end: next.end.max(current_item.end),
                }
            } else {
                merged.push(current_item);
                current_item = next;
            }
        }
        merged.push(current_item);
        merged
    } else {
        Default::default()
    }
}
fn get_mapped_range_from(base: ItemRange, row: &RowMapping) -> Option<ItemRange> {
    let source_range = row.create_source_range();
    let mut junction = get_junction_of_ranges(source_range, base)?;
    let (start, end) = (
        row.dest_start + (junction.start - row.source_start),
        row._dest_end - (row.source_end - junction.end),
    );
    junction.start = start;
    junction.end = end;
    Some(junction)
}
fn get_junction_of_ranges(base: ItemRange, other: ItemRange) -> Option<ItemRange> {
    let other_max_before_b_min = other.end < base.start;
    let other_min_after_b_max = other.start > base.end;
    if other_min_after_b_max || other_max_before_b_min {
        None
    } else {
        let start = if base.start > other.start {
            base.start
        } else {
            other.start
        };
        let end = if base.end < other.end {
            base.end
        } else {
            other.end
        };
        Some(ItemRange { start, end })
    }
}
fn parsing(input: &str) -> TableMapping {
    let mut chunks = utils::parsing::chunks_of_non_empty_lines(input).into_iter();
    let seeds = {
        let line = chunks.next().unwrap();
        line.into_iter()
            .next()
            .unwrap()
            .split(":")
            .skip(1)
            .next()
            .unwrap()
            .trim()
            .split_whitespace()
            .map(|number| number.parse::<NumericVal>().unwrap())
            .collect()
    };

    return TableMapping {
        seeds,
        seeds_to_soil: parse_next_table(chunks.next().unwrap()),
        soil_to_fertilizer: parse_next_table(chunks.next().unwrap()),
        fertilizer_to_water: parse_next_table(chunks.next().unwrap()),
        water_to_light: parse_next_table(chunks.next().unwrap()),
        light_to_temperature: parse_next_table(chunks.next().unwrap()),
        temperature_to_humidity: parse_next_table(chunks.next().unwrap()),
        humidity_to_location: parse_next_table(chunks.next().unwrap()),
    };
    fn parse_next_table(lines: Vec<&str>) -> ImmutableSeq<RowMapping> {
        let lines = lines.into_iter().skip(1);
        let mut to_return: Vec<RowMapping> = lines
            .map(|line| {
                let mut numbers = line
                    .split_whitespace()
                    .map(|number| number.parse::<NumericVal>().unwrap());
                let (dest_start, source_start, _range_len) = (
                    numbers.next().unwrap(),
                    numbers.next().unwrap(),
                    numbers.next().unwrap(),
                );
                RowMapping::new(dest_start, source_start, _range_len)
            })
            .collect();
        to_return.sort_by_key(|key| (key.source_start, Reverse(key._range_len)));
        ImmutableSeq::from(to_return)
    }
}

#[derive(Debug)]
pub struct RowMapping {
    dest_start: NumericVal,
    source_start: NumericVal,
    _range_len: NumericVal,
    _dest_end: NumericVal,
    source_end: NumericVal,
}
impl RowMapping {
    fn new(dest_start: NumericVal, source_start: NumericVal, _range_len: NumericVal) -> Self {
        let _dest_end = (dest_start + _range_len) - 1;
        let source_end = (source_start + _range_len) - 1;
        RowMapping {
            dest_start,
            source_start,
            _range_len,
            _dest_end,
            source_end,
        }
    }

    fn create_source_range(&self) -> ItemRange {
        ItemRange {
            start: self.source_start,
            end: self.source_end,
        }
    }
}

#[derive(Debug)]
pub struct TableMapping {
    seeds: ImmutableNumberSeq,
    seeds_to_soil: ImmutableSeq<RowMapping>,
    soil_to_fertilizer: ImmutableSeq<RowMapping>,
    fertilizer_to_water: ImmutableSeq<RowMapping>,
    water_to_light: ImmutableSeq<RowMapping>,
    light_to_temperature: ImmutableSeq<RowMapping>,
    temperature_to_humidity: ImmutableSeq<RowMapping>,
    humidity_to_location: ImmutableSeq<RowMapping>,
}

#[cfg(test)]
mod testing {

    use super::*;

    #[test]
    fn day_5_get_opt_range_from_row() {
        let row = RowMapping::new(52, 50, 48);
        assert_case(79, &row, Some(81));
        assert_case(100, &row, None);
        assert_case(49, &row, None);
        assert_case(99, &row, None);
        assert_case(50, &row, Some(52));
        assert_case(97, &row, Some(99));
        fn assert_case(val: NumericVal, row: &RowMapping, expected: Option<NumericVal>) {
            let actual = get_withing_range(val, row);
            assert_eq!(expected, actual, "val: {}\n row: {:#?}", val, row);
        }
    }
    #[test]
    fn day_5_get_mapping() {
        let mapping: ImmutableSeq<RowMapping> = Rc::from(vec![
            RowMapping::new(50, 98, 2),
            RowMapping::new(52, 50, 48),
        ]);
        assert_case(13, mapping.clone(), 13);
        assert_case(79, mapping.clone(), 81);
        assert_case(55, mapping.clone(), 57);
        assert_case(14, mapping.clone(), 14);
        fn assert_case(val: NumericVal, mapping: ImmutableSeq<RowMapping>, expected: NumericVal) {
            let actual = get_mapping(val, &mapping);
            assert_eq!(expected, actual, "Val: {}\nmapping: {:#?}", val, mapping);
        }
    }
    #[test]
    fn day_5_merge_ranges() {
        assert_case(vec![], vec![]);
        assert_case(
            vec![ItemRange { start: 2, end: 3 }],
            vec![ItemRange { start: 2, end: 3 }],
        );
        assert_case(
            vec![
                ItemRange { start: 2, end: 5 },
                ItemRange { start: 4, end: 8 },
            ],
            vec![ItemRange { start: 2, end: 8 }],
        );
        assert_case(
            vec![
                ItemRange { start: 2, end: 5 },
                ItemRange { start: 12, end: 13 },
                ItemRange { start: 4, end: 8 },
                ItemRange { start: 13, end: 18 },
                ItemRange { start: 12, end: 14 },
            ],
            vec![
                ItemRange { start: 2, end: 8 },
                ItemRange { start: 12, end: 18 },
            ],
        );
        assert_case(
            vec![
                ItemRange { start: 2, end: 5 },
                ItemRange { start: 6, end: 12 },
                ItemRange { start: 13, end: 15 },
                ItemRange { start: 18, end: 20 },
            ],
            vec![
                ItemRange { start: 2, end: 15 },
                ItemRange { start: 18, end: 20 },
            ],
        );

        fn assert_case(input: Vec<ItemRange>, expected: Vec<ItemRange>) {
            let actual = merge_ranges(input);
            assert_eq!(expected, actual);
        }
    }
    #[test]
    fn day_5_get_junction() {
        assert_case(
            ItemRange { start: 1, end: 2 },
            ItemRange { start: 4, end: 8 },
            None,
        );
        assert_case(
            ItemRange { start: 1, end: 2 },
            ItemRange { start: 3, end: 6 },
            None,
        );
        assert_case(
            ItemRange { start: 1, end: 4 },
            ItemRange { start: 4, end: 6 },
            Some(ItemRange { start: 4, end: 4 }),
        );
        assert_case(
            ItemRange { start: 4, end: 8 },
            ItemRange { start: 6, end: 12 },
            Some(ItemRange { start: 6, end: 8 }),
        );
        assert_case(
            ItemRange { start: 6, end: 12 },
            ItemRange { start: 4, end: 8 },
            Some(ItemRange { start: 6, end: 8 }),
        );

        fn assert_case(left: ItemRange, right: ItemRange, expected: Option<ItemRange>) {
            let actual = get_junction_of_ranges(left, right);
            assert_eq!(expected, actual, "Left: {:?}\nRight: {:?}", left, right);
        }
    }
    #[test]
    fn day_5_get_mapped_range_from() {
        assert_case(
            ItemRange { start: 98, end: 99 },
            &RowMapping::new(50, 98, 2),
            Some(ItemRange { start: 50, end: 51 }),
        );
        assert_case(
            ItemRange { start: 50, end: 74 },
            &RowMapping::new(52, 50, 48),
            Some(ItemRange { start: 52, end: 76 }),
        );
        assert_case(
            ItemRange {
                start: 95,
                end: 100,
            },
            &RowMapping::new(52, 50, 48),
            Some(ItemRange { start: 97, end: 99 }),
        );
        assert_case(
            ItemRange { start: 60, end: 79 },
            &RowMapping::new(60, 50, 30),
            Some(ItemRange { start: 70, end: 89 }),
        );
        assert_case(
            ItemRange { start: 30, end: 45 },
            &RowMapping::new(18, 25, 70),
            Some(ItemRange { start: 23, end: 38 }),
        );
        fn assert_case(base: ItemRange, row: &RowMapping, expected: Option<ItemRange>) {
            let actual = get_mapped_range_from(base, row);
            assert_eq!(expected, actual, "Base: {:?}\nRow: {:?}", base, row);
        }
    }
}

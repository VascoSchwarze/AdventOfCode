use std::cmp::max;
use std::cmp::min;

use regex::Regex;

#[derive(Debug)]
struct RangeMapping {
    source_range: Range,
    destination_range_start: u64,
}

impl RangeMapping {
    fn map_overlap(&self, range: &Range) -> Option<Range> {
        let overlap = match range.get_overlap_with(&self.source_range) {
            None => return None,
            Some(ol) => ol,
        };

        Some(Range {
            start: overlap.start + self.destination_range_start - self.source_range.start,
            length: overlap.length,
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct Range {
    start: u64,
    length: u64,
}

impl Range {
    fn get_end(&self) -> u64 {
        self.start + self.length - 1
    }

    fn remove_overlap_with(first: &Range, other: &Range) -> Vec<Range> {
        let overlap = match first.get_overlap_with(other) {
            None => return vec![*first],
            Some(ol) => ol,
        };

        let mut ranges: Vec<Range> = vec![];
        if overlap.start > first.start {
            ranges.push(Range {
                start: first.start,
                length: overlap.start - first.start,
            });
        };

        let first_end = first.get_end();
        let overlap_end = overlap.get_end();
        if overlap_end < first_end {
            ranges.push(Range {
                start: overlap_end + 1,
                length: first_end - overlap_end,
            });
        };
        ranges
    }

    fn get_overlap_with(&self, other: &Range) -> Option<Range> {
        let start_self = self.start;
        let start_other = other.start;
        let end_self = self.get_end();
        let end_other = other.get_end();

        if start_other > end_self || start_self > end_other {
            return None;
        }

        let overlap_start = max(start_other, start_self);
        let overlap_end = min(end_self, end_other);

        let overlap_length = overlap_end - overlap_start + 1;

        Some(Range {
            start: overlap_start,
            length: overlap_length,
        })
    }
}

fn main() {
    // At the end of the input, two blank lines are required for this solution to work.
    let input = include_str!("../../input.txt");
    // let input = include_str!("../../input_test.txt");
    println!("{input}\n");

    let lines: Vec<&str> = input.lines().collect();

    let numbers_re = Regex::new(r"\d+").unwrap();

    let number_pair_re = Regex::new(r"\d+ \d+").unwrap();
    let mut seeds: Vec<Range> = number_pair_re
        .find_iter(lines[0])
        .map(|mat| {
            let nums: Vec<u64> = mat
                .as_str()
                .split(' ')
                .map(|str| str.parse::<u64>().expect("Should be a number."))
                .collect();
            Range {
                start: nums[0],
                length: nums[1],
            }
        })
        .collect();

    let mut target_ranges: Vec<Range> = vec![];
    let mut range_mappings: Vec<RangeMapping> = vec![];
    for (line_nr, line) in lines.into_iter().enumerate() {
        if (0..=2).contains(&line_nr) || line.contains("map") {
            continue;
        }
        if line.is_empty() {
            for range_mapping in &range_mappings {
                let mut next_seeds: Vec<Range> = vec![];
                for seed in &seeds {
                    let mut remainder =
                        Range::remove_overlap_with(seed, &range_mapping.source_range);
                    next_seeds.append(&mut remainder);
                    if let Some(mapped_overlap) = range_mapping.map_overlap(seed) {
                        target_ranges.push(mapped_overlap)
                    }
                }
                seeds = next_seeds;
            }
            target_ranges.append(&mut seeds);
            seeds = target_ranges.clone();
            target_ranges = vec![];
            range_mappings = vec![];
            continue;
        }

        let range_nums: Vec<u64> = numbers_re
            .find_iter(line)
            .map(|mat| mat.as_str().parse::<u64>().expect("Should be a number."))
            .collect();

        let source_range_start = range_nums[1];
        let dest_range_start = range_nums[0];
        let range_len = range_nums[2];

        range_mappings.push(RangeMapping {
            source_range: Range {
                start: source_range_start,
                length: range_len,
            },
            destination_range_start: dest_range_start,
        });
    }

    let smallest = seeds
        .iter()
        .map(|range| range.start)
        .min()
        .expect("Should have a value");
    println!("{smallest}");
}

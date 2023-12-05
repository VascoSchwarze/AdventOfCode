use regex::Regex;

#[derive(Debug)]
struct RangeMapping {
    range_length: u64,
    source_range_start: u64,
    destination_range_start: u64,
}

impl RangeMapping {
    fn map_if_contained(&self, num: &u64) -> Option<u64> {
        if (self.source_range_start..(self.source_range_start + self.range_length)).contains(num) {
            return Some((num + self.destination_range_start) - self.source_range_start);
        }
        None
    }
}

fn main() {
    let input = include_str!("../../input.txt");
    // let input = include_str!("../../input_test.txt");
    println!("{input}\n");

    let lines: Vec<&str> = input.lines().collect();

    let numbers_re = Regex::new(r"\d+").unwrap();

    let mut seeds: Vec<u64> = numbers_re
        .find_iter(lines[0])
        .map(|mat| mat.as_str().parse::<u64>().expect("Should be a number."))
        .collect();

    let mut target_nums: Vec<u64> = vec![];
    let mut range_mappings: Vec<RangeMapping> = vec![];
    for (line_nr, line) in lines.into_iter().enumerate() {
        if (0..=2).contains(&line_nr) || line.contains("map") {
            continue;
        }
        if line.len() == 0 {
            // println!("{line_nr}");
            // dbg!(&seeds);
            // dbg!(&range_mappings);
            for seed in &seeds {
                let mut mapped = false;
                for range in &range_mappings {
                    match range.map_if_contained(seed) {
                        None => continue,
                        Some(num) => {
                            target_nums.push(num);
                            mapped = true;
                            break;
                        }
                    }
                }
                if !mapped {
                    target_nums.push(seed.clone());
                }
            }
            seeds = target_nums.clone();
            target_nums = vec![];
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
            range_length: range_len,
            source_range_start,
            destination_range_start: dest_range_start,
        });
    }
    
    let smallest = seeds.iter().min().expect("Should have a value.");
    println!("{smallest}");
}

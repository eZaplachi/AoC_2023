use std::{cmp::max, cmp::min, ops::Range, str::SplitWhitespace};

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i64 {
    // Parsing text
    let mut current_title = "";
    let mut seed_nums: Vec<i64> = vec![];
    let mut seed_to_soil: Vec<(Range<i64>, i64)> = vec![];
    let mut soil_to_fertilizer: Vec<(Range<i64>, i64)> = vec![];
    let mut fertilizer_to_water: Vec<(Range<i64>, i64)> = vec![];
    let mut water_to_light: Vec<(Range<i64>, i64)> = vec![];
    let mut light_to_temp: Vec<(Range<i64>, i64)> = vec![];
    let mut temp_to_humidity: Vec<(Range<i64>, i64)> = vec![];
    let mut humidity_to_location: Vec<(Range<i64>, i64)> = vec![];
    for line in input.split("\n").filter(|x| !x.is_empty()) {
        let mut split_line = line.split_whitespace();
        if current_title.is_empty() {
            seed_nums = split_line
                .clone()
                .skip(1)
                .map(|n| n.parse::<i64>().unwrap())
                .collect()
        }
        // Find Title for current map
        if !"12345667890".contains(line.chars().next().unwrap()) {
            current_title = split_line.next().unwrap();
            continue;
        }
        // Get map by title
        if current_title == "seed-to-soil" {
            seed_to_soil.push(map_to_range(split_line.clone()));
        } else if current_title == "soil-to-fertilizer" {
            soil_to_fertilizer.push(map_to_range(split_line.clone()));
        } else if current_title == "fertilizer-to-water" {
            fertilizer_to_water.push(map_to_range(split_line.clone()));
        } else if current_title == "water-to-light" {
            water_to_light.push(map_to_range(split_line.clone()));
        } else if current_title == "light-to-temperature" {
            light_to_temp.push(map_to_range(split_line.clone()));
        } else if current_title == "temperature-to-humidity" {
            temp_to_humidity.push(map_to_range(split_line.clone()));
        } else if current_title == "humidity-to-location" {
            humidity_to_location.push(map_to_range(split_line));
        }
    }
    // Slow brute force solving - could potentially flatten maps into an input = output
    // map for a much more efficient solve
    let mut location_numbers: Vec<i64> = vec![];
    for seed_num in seed_nums {
        let mut out_num = seed_num;
        out_num = apply_offset(out_num, seed_to_soil.clone());
        out_num = apply_offset(out_num, soil_to_fertilizer.clone());
        out_num = apply_offset(out_num, fertilizer_to_water.clone());
        out_num = apply_offset(out_num, water_to_light.clone());
        out_num = apply_offset(out_num, light_to_temp.clone());
        out_num = apply_offset(out_num, temp_to_humidity.clone());
        out_num = apply_offset(out_num, humidity_to_location.clone());
        location_numbers.push(out_num);
    }
    *location_numbers.iter().min().unwrap()
}
fn map_to_range(mut input: SplitWhitespace) -> (Range<i64>, i64) {
    let output_start = option_str_to_num(input.next());
    let input_start = option_str_to_num(input.next());
    // Length of range includes 0 index
    let range_len = option_str_to_num(input.next());
    (
        input_start..input_start + range_len,
        output_start - input_start,
    )
}

fn option_str_to_num(input: Option<&str>) -> i64 {
    input.unwrap().parse::<i64>().unwrap()
}

fn apply_offset(input: i64, input_ranges: Vec<(Range<i64>, i64)>) -> i64 {
    for input_range in input_ranges {
        if input_range.0.contains(&input) {
            return input + input_range.1;
        }
    }
    return input;
}

// Fn to flatten maps into input = output map by splitting overlapping ranges, then adding the new ranges and unused ranges into input = current_step map
// - Couldn't figure it out with time I wanted to spend -
// Compare first range output to second range input
// fn split_overlapping_ranges(
//     first_ranges: Vec<[Range<i64>; 2]>,
//     second_ranges: Vec<[Range<i64>; 2]>,
// ) -> HashSet<[Range<i32>; 2]> {
//     for first_range in first_ranges {
//         for second_range in second_ranges.clone() {
//             //             println!(".");
//             let first_range_min = first_range[1].clone().min().unwrap();
//             let first_range_max = first_range[1].clone().max().unwrap();
//             let second_range_min = second_range[0].clone().min().unwrap();
//             let second_range_max = second_range[0].clone().max().unwrap();
//             if first_range_min <= second_range_max && second_range_min <= first_range_max {
//                 let overlap_range_min = max(first_range_min, second_range_min);
//                 let overlap_range_max = min(first_range_max, second_range_max);
//                 println!("{:?}, {:?}", first_range, second_range);
//             } else {
//                 println!("Non-Overlap {:?}, {:?}", first_range, second_range);
//             }
//         }
//     }
// }

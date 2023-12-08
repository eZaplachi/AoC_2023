fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

// Find sum of hidden numbers in each line
fn part2(input: &str) -> i32 {
    let lines: Vec<_> = input.split_terminator("\n").collect();
    let mut line_results: Vec<i32> = vec![];
    for line in lines {
        let mut current_line_numbers: Vec<(usize, String)> = vec![];
        // Find written numbers and indexes
        // Including if there are multiple of same number

        for text_num in NUMBERARRAY {
            let found_numbers: Vec<_> = line.match_indices(text_num).collect();
            for found_number in &found_numbers {
                match found_number.1 {
                    "one" => current_line_numbers.push((found_number.0, "1".to_string())),
                    "two" => current_line_numbers.push((found_number.0, "2".to_string())),
                    "three" => current_line_numbers.push((found_number.0, "3".to_string())),
                    "four" => current_line_numbers.push((found_number.0, "4".to_string())),
                    "five" => current_line_numbers.push((found_number.0, "5".to_string())),
                    "six" => current_line_numbers.push((found_number.0, "6".to_string())),
                    "seven" => current_line_numbers.push((found_number.0, "7".to_string())),
                    "eight" => current_line_numbers.push((found_number.0, "8".to_string())),
                    "nine" => current_line_numbers.push((found_number.0, "9".to_string())),
                    _ => println!("Error Somewhere"),
                }
            }
        }

        // Find numbers and char index
        for (i, c) in line.chars().enumerate() {
            if INTARRAY.contains(&c) {
                current_line_numbers.push((i, c.to_string()));
            }
        }

        // Compare Indexes to find first and last value
        let mut first_value: (usize, String) = (999, "0".to_string());
        let mut last_value: (usize, String) = (0, "0".to_string());
        for value in current_line_numbers {
            if value.0 < first_value.0 {
                first_value = value.clone();
            }
            if value.0 >= last_value.0 {
                last_value = value;
            }
        }

        let calibration_value = (first_value.1 + &last_value.1).parse::<i32>().unwrap();
        println!("{}", calibration_value);
        line_results.push(calibration_value);
    }
    line_results.iter().sum()
}

const INTARRAY: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];
const NUMBERARRAY: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let test_input = include_str!("./test_input2.txt");
        let result = part2(test_input);
        assert_eq!(result, 281)
    }

    #[test]
    fn found_broken() {
        let test_input = "kpzfgpxdonesix2fourninefourfour";
        let result = part2(test_input);
        assert_eq!(result, 14)
    }
}

use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    let lines = input.split_terminator("\n");
    let mut points_per_line: Vec<i32> = vec![];
    for line in lines {
        // Find winning numbers by matching nums before and after the "|"
        let card_label_removed = line.split(":").last().unwrap().trim();
        let split_card: Vec<_> = card_label_removed.split("|").collect();
        let winning_numbers = str_to_int_hashset(split_card[0]);
        let given_numbers = str_to_int_hashset(split_card[1]);
        // Unsure if bitwise and or iter.filter is better...
        // let matching_numbers: HashSet<i32> = &winning_numbers & &given_numbers;
        let matching_numbers: HashSet<i32> = winning_numbers
            .into_iter()
            .filter(|win_num| given_numbers.contains(win_num))
            .collect();
        // Points are given: 1 for the first and doubles for each subsequent match (ie 2^[matches-1]); 0 points are not added to vec
        if !matching_numbers.is_empty() {
            let num_of_matches: u32 = matching_numbers.len() as u32;
            points_per_line.push(i32::pow(2, num_of_matches - 1));
        }
    }
    points_per_line.iter().sum()
}

fn str_to_int_hashset(input: &str) -> HashSet<i32> {
    input
        .trim()
        .split_whitespace()
        .map(|num| num.parse::<i32>().unwrap())
        .collect::<HashSet<i32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let test_input = include_str!("./test_input1.txt");
        let result = part1(test_input);
        assert_eq!(result, 13)
    }
}

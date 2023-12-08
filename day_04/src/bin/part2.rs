use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> i32 {
    let lines = input.split_terminator("\n");
    let num_of_lines: usize = lines.clone().count();
    let mut copies_of_cards: Vec<i32> = vec![0; num_of_lines];
    for (i, line) in lines.enumerate() {
        // Find winning numbers by matching numbers before and after the "|"
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
        // Add 1 copy of card per game(line), and one copy of to the subsequent cards per match
        // (eg. 2 matches add a copy to the next 2 cards); each copy of card also counts towards
        // adding subsequent copies
        copies_of_cards[i] += 1;
        if !matching_numbers.is_empty() {
            let copies_won = matching_numbers.len();
            for copy_won in 1..copies_won + 1 {
                copies_of_cards[i + copy_won] += 1 * copies_of_cards[i];
            }
        }
    }
    copies_of_cards.iter().sum()
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
        let result = part2(test_input);
        assert_eq!(result, 30)
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> i32 {
    let lines = input.split_terminator("\n");
    let mut symbol_positions: Vec<[i32; 2]> = vec![];
    let mut number_positions: Vec<[i32; 2]> = vec![];
    let mut prev_char = '.';
    let mut current_number = String::new();
    let mut numbers_with_positions: Vec<(i32, Vec<[i32; 2]>)> = vec![];
    // Collect numbers and their char positions by adding number chars between symbols
    for (line_index, line) in lines.enumerate() {
        // Append line with "." so numbers at end of line can be included
        let appended_line = line.to_string() + ".";
        for (column_index, c) in appended_line.chars().enumerate() {
            if SYMBOLS == c {
                symbol_positions.push([line_index as i32, column_index as i32]);
            }
            if NUMBERS.contains(&c) {
                number_positions.push([line_index as i32, column_index as i32]);
                current_number = format!("{}{}", current_number, c);
            }
            if NUMBERS.contains(&prev_char) && !NUMBERS.contains(&c) {
                numbers_with_positions.push((
                    current_number.parse::<i32>().unwrap(),
                    number_positions.clone(),
                ));
                number_positions = vec![];
                current_number = String::new();
            }
            prev_char = c;
        }
    }

    let mut vaid_gear_ratios: Vec<i32> = vec![];
    for symbol_pos in symbol_positions {
        let mut adjecent_ammount = 0;
        let mut adjacent_numbers: Vec<i32> = vec![];
        for number_with_pos in numbers_with_positions.clone() {
            'numbers: for number_pos in number_with_pos.1 {
                // Find if a symbol is within 1 column or line; if so don't check other char
                // positions for
                // the same number
                let line_dist_to_symbol = i32::abs(number_pos[0] - symbol_pos[0]);
                let column_dist_to_symbol = i32::abs(number_pos[1] - symbol_pos[1]);
                if line_dist_to_symbol <= 1 && column_dist_to_symbol <= 1 {
                    adjecent_ammount += 1;
                    adjacent_numbers.push(number_with_pos.0);
                    break 'numbers;
                }
            }
        }
        if adjecent_ammount == 2 {
            let gear_ratio = adjacent_numbers[0] * adjacent_numbers[1];
            vaid_gear_ratios.push(gear_ratio);
        }
    }
    vaid_gear_ratios.iter().sum()
}

const SYMBOLS: char = '*';
const NUMBERS: [char; 10] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let input = include_str!("./test_input1.txt");
        let output = part2(input);
        assert_eq!(output, 467835)
    }
}

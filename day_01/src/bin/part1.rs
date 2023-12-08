fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

// Find sum of hidden numbers in each line
fn part1(input: &str) -> i32 {
    let lines: Vec<_> = input.split_terminator("\n").collect();
    let mut line_results: Vec<i32> = vec![];
    for line in lines {
        let mut current_line_numbers: Vec<String> = vec![];
        for c in line.chars() {
            if INTARRAY.contains(&c) {
                current_line_numbers.push(c.to_string());
            }
        }
        // Get hidden 2-digit numbers for each line - gets the first digit from first number and
        // second digit for last number in Left to Right order; if only one number used for both resulting digits
        let current_line_num_legth = current_line_numbers.len();
        if current_line_num_legth == 1 {
            line_results.push((current_line_numbers[0].repeat(2)).parse::<i32>().unwrap())
        } else if current_line_num_legth > 1 {
            let current_line_res: i32 = (current_line_numbers.first().unwrap().to_owned()
                + current_line_numbers.last().unwrap())
            .parse()
            .unwrap();
            line_results.push(current_line_res);
        }
    }
    line_results.iter().sum()
}

const INTARRAY: [char; 9] = ['1', '2', '3', '4', '5', '6', '7', '8', '9'];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let test_input = include_str!("./test_input1.txt");
        let result = part1(test_input);
        assert_eq!(result, 142)
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input, PART1_MAX);
    dbg!(output);
}

fn part1(input: &str, max_values: [i32; 3]) -> i32 {
    let lines: Vec<_> = input.split_terminator("\n").collect();
    let mut valid_line_ids: Vec<i32> = vec![];
    for line in lines {
        let split_line: Vec<_> = line.split(":").collect();
        let line_id = split_line[0]
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<i32>()
            .unwrap();
        // Check if colors are valid for each game on a line
        let split_games: Vec<_> = split_line[1].split(";").collect();
        let mut highest_reds = 0;
        let mut highest_greens = 0;
        let mut highest_blues = 0;
        for games in split_games {
            let split_colors: Vec<_> = games.split(",").collect();
            for colors in split_colors {
                let num_of_color = colors
                    .trim()
                    .split_whitespace()
                    .next()
                    .unwrap()
                    .parse::<i32>()
                    .unwrap();
                if colors.contains("red") && num_of_color > highest_reds {
                    highest_reds = num_of_color;
                }
                if colors.contains("green") && num_of_color > highest_greens {
                    highest_greens = num_of_color;
                }
                if colors.contains("blue") && num_of_color > highest_blues {
                    highest_blues = num_of_color;
                }
            }
        }
        if max_values[0] >= highest_reds
            && max_values[1] >= highest_greens
            && max_values[2] >= highest_blues
        {
            valid_line_ids.push(line_id);
        }
    }
    valid_line_ids.iter().sum()
}

// Max values in R-G-B order
const PART1_MAX: [i32; 3] = [12, 13, 14];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let input = include_str!("./test_input1.txt");
        let output = part1(input, PART1_MAX);
        assert_eq!(output, 8)
    }
}

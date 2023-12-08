fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> i32 {
    let lines: Vec<_> = input.split_terminator("\n").collect();
    let mut line_powers: Vec<i32> = vec![];
    for line in lines {
        // Find powers for each line by multiplying the minimum required of each color together
        let split_games: Vec<_> = line.split(":").last().unwrap().split(";").collect();
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
        line_powers.push(highest_reds * highest_greens * highest_blues);
    }
    line_powers.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works() {
        let input = include_str!("./test_input1.txt");
        let output = part2(input);
        assert_eq!(output, 2286)
    }
}

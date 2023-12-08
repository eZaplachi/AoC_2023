fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> i64 {
    // Parsing - get array with time in position 0 and distance in position 1
    let mut time_string: String = String::new();
    let mut distance_string: String = String::new();
    for (line_i, line) in input.split_terminator("\n").enumerate() {
        let split_line = line.split_whitespace().filter(|x| !x.is_empty()).skip(1);
        for nums in split_line {
            if line_i == 0 {
                time_string = time_string + nums;
            } else {
                distance_string = distance_string + nums;
            }
        }
    }
    let time: i64 = time_string.parse().unwrap();
    let distance: i64 = distance_string.parse().unwrap();

    // Calculation
    let mut ways_to_win = 0;
    for charge_time in 0..time {
        let winning_distance = (time - charge_time) * charge_time;
        if winning_distance > distance {
            ways_to_win += 1;
        }
    }
    ways_to_win
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./test_input1.txt");
        let output = part2(input);
        assert_eq!(output, 71503)
    }
}

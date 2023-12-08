fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    // Parsing - get arrays with time in position 0 and distance in position 1
    let mut times_to_distances: Vec<[i32; 2]> = vec![];
    let mut times: Vec<i32> = vec![];
    for (line_i, line) in input.split_terminator("\n").enumerate() {
        let split_line = line
            .split_whitespace()
            .filter(|x| !x.is_empty())
            .skip(1)
            .map(|n| n.parse::<i32>());
        for (nums_i, nums) in split_line.enumerate() {
            if line_i == 0 {
                times.push(nums.unwrap());
            } else {
                times_to_distances.push([times[nums_i], nums.unwrap()])
            }
        }
    }
    // Calculation
    let mut ways_to_win_multiplied: i32 = 1;
    for time_to_distance in times_to_distances {
        let mut ways_to_win = 0;
        for charge_time in 0..time_to_distance[0] {
            let distance = (time_to_distance[0] - charge_time) * charge_time;
            if distance > time_to_distance[1] {
                ways_to_win += 1;
            }
        }
        ways_to_win_multiplied = ways_to_win_multiplied * ways_to_win
    }
    ways_to_win_multiplied
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("./test_input1.txt");
        let output = part1(input);
        assert_eq!(output, 288)
    }
}

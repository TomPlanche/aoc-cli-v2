use utils::{run_solution, Solution};

struct Day{DAY};

impl Solution for Day{DAY} {
    type Input = Vec<i32>;
    type Output = i32;

    fn parse_input(&self, input: &str) -> Self::Input {
        input
            .lines()
            .filter_map(|line| line.parse().ok())
            .collect()
    }

    fn part1(&self, _data: &Self::Input) -> Self::Output {
        // TODO: Implement part 1
        0
    }

    fn part2(&self, _data: &Self::Input) -> Self::Output {
        // TODO: Implement part 2
        0
    }
}

fn main() {
    run_solution!(Day{DAY});
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "";

        let day = Day{DAY};
        let parsed_input = day.parse_input(input);

        let part1 = day.part1(&parsed_input);

        assert_eq!(part1, 0);
    }

    #[test]
    fn test_part2() {
        let input = "";

        let day = Day{DAY};
        let parsed_input = day.parse_input(input);

        let part2 = day.part2(&parsed_input);

        assert_eq!(part2, 0);
    }
}

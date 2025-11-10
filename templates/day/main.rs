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

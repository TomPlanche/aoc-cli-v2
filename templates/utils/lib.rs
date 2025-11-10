pub mod directions;
pub mod points;

pub use directions::*;
pub use points::*;

pub trait Solution {
    type Input;
    type Output: std::fmt::Display;

    fn parse_input(&self, input: &str) -> Self::Input;
    fn part1(&self, input: &Self::Input) -> Self::Output;
    fn part2(&self, input: &Self::Input) -> Self::Output;

    fn solve(&self, input: &str) {
        let parsed = self.parse_input(input);

        println!("Part 1: {}", self.part1(&parsed));
        println!("Part 2: {}", self.part2(&parsed));
    }
}

#[macro_export]
macro_rules! run_solution {
    ($solution:expr) => {{
        let input = include_str!("../input.txt");
        $solution.solve(input);
    }};
}

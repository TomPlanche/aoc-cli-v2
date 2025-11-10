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

    fn solve_timed(&self, input: &str) -> (std::time::Duration, std::time::Duration) {
        use std::time::Instant;

        let parsed = self.parse_input(input);

        let start1 = Instant::now();
        let result1 = self.part1(&parsed);
        let time1 = start1.elapsed();

        let start2 = Instant::now();
        let result2 = self.part2(&parsed);
        let time2 = start2.elapsed();

        println!("Part 1: {} ({}ms)", result1, time1.as_secs_f64() * 1000.0);
        println!("Part 2: {} ({}ms)", result2, time2.as_secs_f64() * 1000.0);
        println!("TIMING:{}:{}", time1.as_micros(), time2.as_micros());

        (time1, time2)
    }
}

#[macro_export]
macro_rules! run_solution {
    ($solution:expr) => {{
        let args: Vec<String> = std::env::args().collect();
        let timing_mode = args.get(1).map_or(false, |arg| arg == "--timing");

        let input = include_str!("../input.txt");

        if timing_mode {
            $solution.solve_timed(input);
        } else {
            $solution.solve(input);
        }
    }};
}

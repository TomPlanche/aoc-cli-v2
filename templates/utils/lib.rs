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

        let start1 = Instant::now();
        let parsed = self.parse_input(input);
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

    fn solve_part1_timed(&self, input: &str) -> std::time::Duration {
        use std::time::Instant;

        let start = Instant::now();
        let parsed = self.parse_input(input);
        let result = self.part1(&parsed);
        let elapsed = start.elapsed();

        println!("Part 1: {} ({}ms)", result, elapsed.as_secs_f64() * 1000.0);
        println!("TIMING_PART1:{}", elapsed.as_micros());

        elapsed
    }

    fn solve_part2_timed(&self, input: &str) -> std::time::Duration {
        use std::time::Instant;

        let start = Instant::now();
        let parsed = self.parse_input(input);
        let result = self.part2(&parsed);
        let elapsed = start.elapsed();

        println!("Part 2: {} ({}ms)", result, elapsed.as_secs_f64() * 1000.0);
        println!("TIMING_PART2:{}", elapsed.as_micros());

        elapsed
    }
}

#[macro_export]
macro_rules! run_solution {
    ($solution:expr) => {{
        let args: Vec<String> = std::env::args().collect();
        let mode = args.get(1).map(String::as_str);

        let input = include_str!("../input.txt");

        match mode {
            Some("--timing") => {
                $solution.solve_timed(input);
            }
            Some("--part1") => {
                $solution.solve_part1_timed(input);
            }
            Some("--part2") => {
                $solution.solve_part2_timed(input);
            }
            _ => {
                $solution.solve(input);
            }
        }
    }};
}

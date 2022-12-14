use crate::aoc::aoc_problem::{AoCProblem, Solution};

pub struct Day00 {
    num: usize,
}

// Testing
impl AoCProblem for Day00 {
    fn prepare(input: &str) -> Self {
        let num = input.parse().unwrap();
        Self { num }
    }

    fn part1(&mut self) -> Solution {
        (self.num + my_helper_function()).into()
    }

    fn part2(&mut self) -> Solution {
        (self.num + 1000 + my_helper_function() * 10).into()
    }
}

/// A random helper function that does something.
///
/// # Returns
/// - A value.
fn my_helper_function() -> usize {
    10_usize
}

use std::ops::Deref;
use std::io::{Error, Read};

use adventofcode_2023day_05::solution::Solution;
use adventofcode_2023day_05::Solve;
use utils::file::read_file;


struct PartTwo(Solution);

impl Deref for PartTwo {
    type Target =  Solution;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Solve for PartTwo {
    fn solve(&self) -> u128 {
        let mut i: usize = 0;
        while i  < self.seeds.len() {
            let frm = self.seeds[i];
            let to = frm + self.seeds[i+1];
            i += 2
        }

        todo!();
    }
}


pub fn start() -> Result<(), Error>{
    let mut reader = read_file("solutions/day_05/input.txt").expect("Err");
    let mut input_string = String::new();

    reader.read_to_string(&mut input_string).expect("Failed to read input file");

    let solution =  PartTwo(Solution::create(&input_string));
    
    Ok(())
}
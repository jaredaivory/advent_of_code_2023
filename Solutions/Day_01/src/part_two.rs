use std::collections::HashMap;
use std::fmt::Error;
use std::io::prelude::BufRead;
use utils::file::read_file;

fn calibration_value(calibration: String) -> u32 {
    let mut left = 0;

    let zero_bound =  '0' as u8;
    let nine_bound = '9' as u8;

    let hashMap = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut i = 0;
    let n = calibration.len();
    'first: while i < n {
        if calibration.as_bytes()[i] >= zero_bound && calibration.as_bytes()[i] <= nine_bound {
            left = (calibration.as_bytes()[i] - zero_bound) as u32 * 10;
            i += 1;
            break 'first;
        }
        for (key, value) in &hashMap {
            if calibration[i..n].starts_with(key) {
                left = value * 10;
                i += 1;
                break 'first
            }
        }
        i += 1;
    }
    let mut right = 0;
    let bytes = calibration.as_bytes();
    while i < n {
        if bytes[i] >= zero_bound && bytes[i] <= nine_bound {
            right = (bytes[i] - zero_bound) as u32;
            println!("{}", right)
        }
        else {
            for (key, value) in &hashMap {
                if calibration[i..n].starts_with(key) {
                    right = *value;
                }
            }
        }
        i += 1;
    }


    left + right
}



pub fn start() -> Result<(), Error> {
    let reader = read_file("solutions/day_01/input.txt").expect("Err");

    let mut calibration_value_sum = 0;
    
    for line in reader.lines() {
        match line {
            Ok(line) => {
                calibration_value_sum += calibration_value(line);
            },
            Err(error) => panic!("Problem opening the file: {:?}", error)
        }
    }

    println!("\nAdvent Of Code - 2023 | Part 2: Calibration Sum = {}", calibration_value_sum);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("two1nine" => 29; "test_1")]
    #[test_case("eightwothree" => 83; "test_2")]
    #[test_case("abcone2threexyz" => 13; "test_3")]
    #[test_case("xtwone3four" => 24; "test_4")]
    #[test_case("4nineeightseven2" => 42; "test_5")]
    #[test_case("zoneight234" => 14; "test_6")]
    #[test_case("7pqrstsixteen" => 76; "test_7")]
    fn process_line(s: &str) -> u32 {
        calibration_value(s.to_string())
    }
}
use std::fmt::Error;
use std::io::prelude::BufRead;
use utils::file::read_file;


fn calibration_value(calibration: String) -> u32 {

    let bytes = calibration.as_bytes();

    let mut result = 0;

    let zero_bound =  '0' as u8;
    let nine_bound = '9' as u8;

    for i in 0..bytes.len() {
        if bytes[i] >= zero_bound && bytes[i] <= nine_bound {
            result += (bytes[i] - zero_bound) as u32 * 10;
            break;
        }
    }

    for i in (0..bytes.len()).rev() {
        if bytes[i] >= zero_bound && bytes[i] <= nine_bound {
            result += (bytes[i] - zero_bound) as u32;
            break;
        }
    }
    result
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

    println!("\n\n\nAdvent Of Code - 2023 | Day 01 | 'Trebuchet?!': Part 1 \n Answer: {}", calibration_value_sum);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::calibration_value;
    use test_case::test_case;

    #[test_case("1abc2" => 12; "test_1")]
    #[test_case("pqr3stu8vwx" => 38; "test_2")]
    #[test_case("a1b2c3d4e5f" => 15; "test_3")]
    #[test_case("treb7uchet" => 77; "test_4")]
    fn process_calibration_line(s: &str) -> u32 {
        calibration_value(s.to_string())
    }
}

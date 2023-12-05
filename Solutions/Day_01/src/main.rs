use std::fmt::Error;
use std::io::{self, prelude::BufRead, BufReader};
use std::path::Path;
use std::fs::File;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_file<P>(file_path: P) -> io::Result<io::BufReader<File>>
where P: AsRef<Path>, {
    let file = File::open(file_path)?;
    Ok(BufReader::new(file))
}


fn calibration_value(calibration: String, calibration_sum: &mut u32) {
    let bytes = calibration.as_bytes();

    let mut result: u32 = 0;

    let zero_bound =  "0".as_bytes()[0];
    let nine_bound = "9".as_bytes()[0];
    // Left Value
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
    println!("{} | {}",  calibration, result);
    *calibration_sum += result;
}




fn main() -> Result<(), Error> {
    let reader = read_file("solutions/day_01/input.txt").expect("Err");

    let mut calibration_value_sum: u32 = 0;
    for line in reader.lines() {
        match line {
            Ok(line) => {
                calibration_value(line, &mut calibration_value_sum);
            },
            Err(error) => panic!("Problem opening the file: {:?}", error)
        }
    }

    println!("{}", calibration_value_sum);

    Ok(())
}

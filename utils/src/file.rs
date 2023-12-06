
use std::io::{self, BufReader};
use std::path::Path;
use std::fs::File;


pub fn read_file<P>(file_path: P) -> io::Result<io::BufReader<File>>
where P: AsRef<Path>, {
    let file = File::open(file_path)?;
    Ok(BufReader::new(file))
}
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>, // Trait bound: P must be convertible to &Path
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

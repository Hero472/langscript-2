use std::{fs::File, io::{self, BufRead}, path::Path};

pub fn read_file(filename: &str) -> Vec<char> {
    let mut result = vec![];
    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            result.extend(line.chars());
            result.push('\n');
        }
    }
    result
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
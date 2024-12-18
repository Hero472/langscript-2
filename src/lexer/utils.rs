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
    let file = File::open(filename).expect("File name was not found");
    Ok(io::BufReader::new(file).lines())
}

// Test function for read_file
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;

    #[test]
    fn test_read_file() {
        // Create a temporary test file
        let temp_file_path = "test_file.txt";
        let mut temp_file = File::create(temp_file_path).expect("Failed to create file");

        // Write some test content to the file
        let content = "Hello\nWorld";
        temp_file.write_all(content.as_bytes()).expect("Failed to write to file");

        // Call `read_file` to read the file's contents
        let result = read_file(temp_file_path);

        // The expected output as a Vec<char>
        let expected: Vec<char> = "Hello\nWorld".chars().collect::<Vec<char>>().into_iter().chain(vec!['\n']).collect();

        // Assert that the result matches the expected value
        assert_eq!(result, expected);

        // Clean up the temporary file
        fs::remove_file(temp_file_path).expect("Failed to remove temporary file");
    }
}
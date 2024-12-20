use std::{fs::File, io::{self, BufRead}, path::Path};

pub fn read_file(filename: &str) -> String {
    let mut result = String::new();
    if let Ok(lines) = read_lines(filename) {
        for line in lines.flatten() {
            result.push_str(&line);
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

#[derive(Debug)]
pub struct StringStream {
    input: Vec<char>,
    position: usize,
}

impl StringStream {
    /// Creates a new StringStream from a generic collection.
    pub fn new(input: String) -> Self {
        StringStream {
            input: input.chars().collect(),  // Convert the String into a Vec<char>
            position: 0,
        }
    }

    /// Peeks at the current element without consuming it.
    pub fn peek(&self) -> Option<&char> {
        self.input.get(self.position)
    }

    /// Peeks at the next element without consuming.
    pub fn peek_next(&self) -> Option<&char> {
        self.input.get(self.position + 1)
    }

    /// Consumes the current element and moves the position forward.
    pub fn next(&mut self) -> Option<char> {
        if self.position < self.input.len() {
            let current = self.input[self.position].clone();
            self.position += 1;
            Some(current)
        } else {
            None
        }
    }

    /// Checks if the stream has reached the end.
    pub fn is_eof(&self) -> bool {
        self.position >= self.input.len()
    }
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

        // The expected output as a String
        let expected = "Hello\nWorld\n".to_string();

        // Assert that the result matches the expected value
        assert_eq!(result, expected);

        // Clean up the temporary file
        fs::remove_file(temp_file_path).expect("Failed to remove temporary file");
    }

    #[test]
    fn test_stream_peek_and_next() {
        let input = String::from("String");
        let mut stream = StringStream::new(input);

        // Test initial peek
        assert_eq!(stream.peek(), Some(&'S'));
        assert_eq!(stream.peek_next(), Some(&'t'));

        // Test next
        assert_eq!(stream.next(), Some('S'));
        assert_eq!(stream.peek(), Some(&'t'));
        assert_eq!(stream.peek_next(), Some(&'r'));

        // Consume the rest of the stream
        assert_eq!(stream.next(), Some('t'));
        assert_eq!(stream.next(), Some('r'));
        assert_eq!(stream.next(), Some('i'));
        assert_eq!(stream.next(), Some('n'));
        assert_eq!(stream.next(), Some('g'));

        // Test EOF behavior
        assert_eq!(stream.next(), None);
        assert_eq!(stream.peek(), None);
        assert_eq!(stream.peek_next(), None);
        assert!(stream.is_eof());
    }

    #[test]
    fn test_multi_line_string() {
        let input = String::from("/*\n*/");
        let mut stream = StringStream::new(input);

        assert_eq!(stream.next(), Some('/'));
        assert_eq!(stream.next(), Some('*'));
        assert_eq!(stream.next(), Some('\n'));
        assert_eq!(stream.next(), Some('*'));
        assert_eq!(stream.next(), Some('/'));
    }

    #[test]
    fn test_stream_empty_input() {
        let input = String::new();
        let mut stream = StringStream::new(input);

        assert_eq!(stream.peek(), None);
        assert_eq!(stream.peek_next(), None);
        assert_eq!(stream.next(), None);
        assert!(stream.is_eof());
    }

    #[test]
    fn test_stream_partial_consumption() {
        let input = String::from("ABC");
        let mut stream = StringStream::new(input);

        // Consume part of the stream
        assert_eq!(stream.next(), Some('A'));
        assert_eq!(stream.peek(), Some(&'B'));
        assert_eq!(stream.peek_next(), Some(&'C'));

        // Reset and verify behavior
        assert_eq!(stream.next(), Some('B'));
        assert_eq!(stream.next(), Some('C'));
        assert!(stream.is_eof());
    }
}

use std::io::{self, BufRead, Write};

pub struct Count {
    pub lines: Option<usize>,
    pub words: Option<usize>,
    pub bytes: Option<usize>,
    pub chars: Option<usize>,
}

impl Count {
    pub fn new(lines: bool, words: bool, bytes: bool, chars: bool) -> Self {
        Count {
            lines: if lines { Some(0) } else { None },
            words: if words { Some(0) } else { None },
            bytes: if bytes { Some(0) } else { None },
            chars: if chars { Some(0) } else { None },
        }
    }

    pub fn print_line<T: Write>(&self, filename: &str, writer: &mut T) -> Result<(), io::Error> {
        self.lines
            .map(|line_count| write!(writer, "{:>8}", line_count))
            .transpose()?;
        self.words
            .map(|word_count| write!(writer, "{:>8}", word_count))
            .transpose()?;
        self.bytes
            .map(|byte_count| write!(writer, "{:>8}", byte_count))
            .transpose()?;
        self.chars
            .map(|char_count| write!(writer, "{:>8}", char_count))
            .transpose()?;
        if filename != "-" {
            write!(writer, " {}", filename)?;
        }
        writeln!(writer)?;

        Ok(())
    }

    pub fn sum(&self, other: &Count) -> Count {
        Count {
            lines: self.lines.map(|l| l + other.lines.unwrap_or(0)),
            words: self.words.map(|w| w + other.words.unwrap_or(0)),
            bytes: self.bytes.map(|b| b + other.bytes.unwrap_or(0)),
            chars: self.chars.map(|c| c + other.chars.unwrap_or(0)),
        }
    }
}

pub fn process_line<T: BufRead>(
    mut file: T,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
) -> Result<Count, io::Error> {
    let mut line = String::new();
    let mut result = Count::new(lines, words, bytes, chars);
    loop {
        match file.read_line(&mut line)? {
            0 => break,
            n => {
                result.lines = result.lines.map(|count| count + 1);
                result.words = result
                    .words
                    .map(|count| count + word_counter(line.as_str()));
                result.bytes = result.bytes.map(|count| count + n);
                result.chars = result.chars.map(|count| count + char_counter(&line));
            }
        }
        line.clear();
    }

    Ok(result)
}

pub fn word_counter(line: &str) -> usize {
    line.split_whitespace().count()
}
pub fn byte_counter(line: &str) -> usize {
    line.len()
}
pub fn char_counter(line: &str) -> usize {
    line.chars().count()
}

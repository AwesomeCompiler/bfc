use std::fmt;
use std::ops::Range;
use ansi_term::Colour::{Red,Purple};
use ansi_term::Style;
use ansi_term::ANSIStrings;
use self::Level::*;

#[derive(Debug)]
#[allow(dead_code)]
pub enum Level {
    Warning,
    Error
}

#[derive(Debug)]
pub struct Info {
    pub level: Level,
    pub filename: String,
    pub message: String,
    // from and to (can be the same)
    pub position: Option<Range<usize>>,
    pub source: Option<String>,
}

// Given an index into a string, return the line number and column
// count (both zero-indexed).
fn position(s: &str, i: usize) -> (usize, usize) {
    let mut char_count = 0;
    for (line_idx, line) in s.split('\n').enumerate() {
        let line_length = line.len();
        if char_count + line_length >= i {
            return (line_idx, i - char_count);
        }

        char_count += line_length + 1;
    }

    unreachable!()
}

impl fmt::Display for Info {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        let mut file_text = self.filename.to_owned();

        // Find line and column offsets, if we have an index.
        let offsets = match (&self.position, &self.source) {
            (&Some(ref range), &Some(ref source)) => {
                let (line_idx, column_idx) = position(source, range.start);

                file_text = file_text + &format!(":{}:{}", line_idx + 1, column_idx + 1);
                Some((line_idx, column_idx, range.end - range.start))
            }
            _ => None
        };

        let level_text;
        let color;
        match self.level {
            Warning => {
                color = Purple;
                level_text = " warning: ";
            }
            Error => {
                color = Red;
                level_text = " error: ";
            }
        }

        let mut context_line = "".to_owned();
        let mut caret_line = "".to_owned();
        match (offsets, &self.source) {
            (Some((line_idx, column_idx, width)), &Some(ref source)) => {
                // The faulty line of code.
                let line = source.split('\n').nth(line_idx).unwrap();
                context_line = "\n".to_owned() + &line;

                // Highlight the faulty characters on that line.
                caret_line = caret_line + "\n";
                for _ in 0..column_idx {
                    caret_line = caret_line + " ";
                }
                caret_line = caret_line + "^";
                for _ in 0..(width - 1) {
                    caret_line = caret_line + "~";
                }
            }
            _ => {}
        }

        let bold = Style::new().bold();
        let default = Style::default();
        let strings = [bold.paint(file_text),
                       color.bold().paint(level_text),
                       bold.paint(self.message.clone()),
                       default.paint(context_line),
                       color.bold().paint(caret_line)];
        write!(f, "{}", ANSIStrings(&strings))
    }
}

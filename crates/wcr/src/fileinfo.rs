use std::{error::Error, io::BufRead};

#[derive(Debug, PartialEq, Eq)]
pub struct FileInfo {
    pub lines: usize,
    pub words: usize,
    pub bytes: usize,
    pub chars: usize,
}

pub fn count(mut file: impl BufRead) -> Result<FileInfo, Box<dyn Error>> {
    let mut lines = 0;
    let mut words = 0;
    let mut bytes = 0;
    let mut chars = 0;

    let mut line = String::new();

    loop {
        let line_bytes = file.read_line(&mut line)?;
        if line_bytes == 0 {
            break;
        }
        bytes += line_bytes;
        lines += 1;
        words += line.split_whitespace().count();
        chars += line.chars().count();
        line.clear();
    }

    Ok(FileInfo {
        lines,
        words,
        bytes,
        chars,
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());

        let expected = FileInfo {
            lines: 1,
            words: 10,
            chars: 48,
            bytes: 48,
        };

        assert_eq!(info.unwrap(), expected);
    }
}

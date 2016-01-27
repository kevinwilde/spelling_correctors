// Kevin Wilde
// NETID: kjw731
// EECS 395

#[doc="
Words do not include any characters except alphabetic characters and apostrophes and periods (to allow acronyms).
The program is not case sensitive (all words are converted to lowercase). `Hello` and `hello` count as the same word.
Numbers are not words.
Apostrophes are part of words as long as they are not at the beginning or end. 
Periods are trimmed from the beginning and end of words. Abbreviations like `etc.` will be counted as just `etc`.
Acronyms separated by periods will have the last period removed (ex. `E.E.C.S.` would show up in the output as `e.e.c.s`).
Hyphenated words are split into separate words. Thus, `good-looking` would separate into `good` and `looking`.
"]

use std;
use std::io::{BufRead, BufReader, Read};

pub fn read_input<R: Read>(reader: R) -> Vec<String> {
    let mut v = std::vec::Vec::new();
    let mut lines = BufReader::new(reader).lines();
    while let Some(Ok(line)) = lines.next() {
        let lower_line = line.to_lowercase();
        let tmp: Vec<&str> = lower_line.split(|c: char| !c.is_alphabetic() && c != '\'' && c != '.').collect();
        for elem in tmp {            
            let mut word = &elem[..];
            let mut size = word.len();

            // Remove leading apostrophes and periods
            while size > 0 && (word.chars().nth(0).unwrap() == '\'' || word.chars().nth(0).unwrap() == '.') {
                word = &word[1..];
                size -= 1;
            }

            // Remove trailing apostrophes and periods
            while size > 0 && (word.chars().nth(size-1).unwrap() == '\'' || word.chars().nth(size-1).unwrap() == '.') {
                word = &word[..(size-1)];
                size -= 1;
            }
            
            if word.len() > 0 {
                v.push(word.to_owned());
            }
        }
    }
    v
}

#[cfg(test)]
mod read_input_tests {
    use super::{read_input};
    use std::io::{Read, Result};

    #[test]
    fn reads_three_words_on_separate_lines() {
        assert_read(&["hi", "hello", "hey"], "'...''hi...\nhello\nhey\n");
    }

    #[test]
    fn reads_three_words_on_same_line() {
        assert_read(&["hi", "hello", "hey"], "hi hello hey\n");
    }

    #[test]
    fn trims_beginning_and_trailing_apostrophes_and_periods() {
        assert_read(&["hi", "hello", "hey"], "'...''hi...\n''...'..'hello...'\n'..'..'.hey\n");
    }

    #[test]
    fn handles_acronyms() {
        assert_read(&["i", "am", "a", "student", "in", "the", "e.e.c.s", "department"], "I am a student \nin the E.E.C.S. department!!\n");
    }

    #[test]
    fn splits_on_invalid_chars() {
        assert_read(&["hi", "my", "name", "is", "kevin", "i", "don't", "like", "the", "one"], "hi8 my name&is Kevin. I don't like the # 3.14159. One=1.")
    }

    #[test]
    fn splits_on_invalid_chars_multi_line() {
        assert_read(&["hi", "my", "name", "is", "kevin", "i", "don't", "like", "the", "one"], "hi8 my\nname&is Kevin!!!!\n I don't\nlike the # 3. One=1.\n%$#^$^")
    }

    fn assert_read(expected: &[&str], input: &str) {
        let mock_read = StringReader::new(input.to_owned());
        let v = read_input(mock_read);
        assert_eq!(expected.len(), v.len());
        for i in 0..(v.len()) {
            assert_eq!(expected[i], v[i]);
        }
    }
    
    struct StringReader {
        contents: Vec<u8>,
        position: usize,
    }

    impl StringReader {
        fn new(s: String) -> Self {
            StringReader {
                contents: s.into_bytes(),
                position: 0,
            }
        }
    }

    impl Read for StringReader {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
            let mut count = 0;
            while self.position < self.contents.len() && count < buf.len() {
                buf[count] = self.contents[self.position];
                count += 1;
                self.position += 1;
            }
            Ok(count)
        }
    }
}

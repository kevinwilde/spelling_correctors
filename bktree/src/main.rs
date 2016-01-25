// Kevin Wilde
// NETID: kjw731
// EECS 395

#[doc="

"]

use std::{env, fs};
use std::io::{BufRead, BufReader, Read, stdin};

mod node;
mod bktree;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("Pass in one argument--the name of the training file");
    }
    let f = fs::File::open(&args[1]).expect("Error opening the training file");
    let v: Vec<String> = read_input(f);
    let root = node::Node{word: v[0].to_owned(), freq: 0, children: std::collections::HashMap::new()};
    let mut bk = bktree::BKTree{ root: root };
    for w in &v {
        bk.add(w);
    }
    let input = read_input(stdin());
    for word in &input {
    	let best_suggestion = bk.search(word, &2);
        if best_suggestion.len() > 0 {
   		   println!("{}, {}", word, best_suggestion);
        } else {
            println!("{}, -", word);
        }
    }
}
















fn read_input<R: Read>(reader: R) -> Vec<String> {
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

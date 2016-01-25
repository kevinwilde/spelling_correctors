// Kevin Wilde
// NETID: kjw731
// EECS 395

#[doc="

"]

use std::{env, fs, io};

mod input;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("Pass in one argument--the name of the training file");
    }
	let mut word_library = input::CountTable::new();
    let f = fs::File::open(&args[1]).expect("Error opening the training file");
    input::read_and_count(&mut word_library, f);
    let v: Vec<String> = input::read_input(io::stdin());
    for word in &v {
        if word_library.contains_key(word) {
            println!("{}", word);
        } else {
            println!("{}, {}", word, correct(word, &word_library));
        }
    }
}

const ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyz";

fn edits1(word: &str) -> Vec<String> {
	let word_len = word.len();
	let mut splits = Vec::new();
	let mut v = Vec::new();

	for i in 0..(word_len+1) {
		splits.push((&word[..i], &word[i..]));
	}

	//Deletes
	for t in &splits {		
		if t.1.len() > 0 {
			let new_edit = t.0.to_owned() + &t.1[1..];
            if !v.contains(&new_edit) {
                v.push(new_edit);
            }
		}
	}

	//Transposes
	for t in &splits {
		if t.1.len() > 1 {
            let new_edit = t.0.to_owned() + &t.1[1..2] + &t.1[0..1] + &t.1[2..];
            if !v.contains(&new_edit) {
				v.push(new_edit);
            }
		}
	}

	//Replaces
	for t in &splits {
		if t.1.len() > 0 {
			for i in 0..ALPHABET.len() {
                let new_edit = t.0.to_owned() + &ALPHABET[i..(i+1)] + &t.1[1..];
                if !v.contains(&new_edit) {
    				v.push(new_edit);
                }
			}
		}
	}

	//Inserts
	for t in &splits {
		for i in 0..ALPHABET.len() {
			let new_edit = t.0.to_owned() + &ALPHABET[i..(i+1)] + &t.1[..];
            if !v.contains(&new_edit) {
                v.push(new_edit);
            }
		}
	}
	v
}

#[cfg(test)]
mod edit1_tests {

    use super::{edits1};

    #[test]
    fn edits1_for_a() {
        let word = "a";
        let v = edits1(&word);
        assert_eq!(78, v.len());
    }

    #[test]
    fn edits1_for_e() {
        let word = "e";
        let v = edits1(&word);
        assert_eq!(78, v.len());
    }

    #[test]
    fn edits1_for_aaaaa() {
        let word = "aaaaa";
        let v = edits1(&word);
        assert_eq!(278, v.len());
    }

    #[test]
    fn edits1_for_abc() {
        let word = "abc";
        let v = edits1(&word);
        assert_eq!(182, v.len());
    }

    #[test]
    fn edits1_for_abcde() {
        let word = "abcde";
        let v = edits1(&word);
        assert_eq!(286, v.len());
    }

    #[test]
    fn edits1_for_abcdefghijklmnopqrstuvwxyz() {
        let word = "abcdefghijklmnopqrstuvwxyz";
        let v = edits1(&word);
        assert_eq!(1378, v.len());
    }

    #[test]
    fn edits1_for_hello() {
        let word = "hello";
        let v = edits1(&word);
        assert_eq!(284, v.len());
    }

    #[test]
    fn edits1_for_something() {
        let word = "something";
        let v = edits1(&word);
        assert_eq!(494, v.len());
    }

    #[test]
    fn edits1_for_anything() {
        let word = "anything";
        let v = edits1(&word);
        assert_eq!(442, v.len());
    }

}

fn known(words: Vec<String>, word_library: &input::CountTable) -> Vec<String> {
	let mut v = Vec::new();
	for word in words {
		if word_library.contains_key(&word) {
			v.push(word);
		}
	}
	v
}

fn known_edits2(word: &str, word_library: &input::CountTable) -> Vec<String> {
	let mut v = Vec::new();
	for e1 in edits1(word) {
		for e2 in edits1(&e1) {
			if word_library.contains_key(&e2) {
				v.push(e2);
			}
		}
	}
	v
}

fn correct(word: &str, word_library: &input::CountTable) -> String {
    let mut candidates = known(edits1(word), word_library);
    if candidates.len() <= 0 {
        candidates = known_edits2(word, word_library);
    }
    let mut best_word: String = "-".to_owned();
    let mut best_word_score: usize = 0;
    for word in &candidates {
        let score = word_library.get(word).unwrap();
        if  score > &best_word_score {
            best_word = word.to_owned();
            best_word_score = score.to_owned();
        }
    }
    best_word
}
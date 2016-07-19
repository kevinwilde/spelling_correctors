#[doc="
Provides functions to
  * Compute the possible edits to a word, where an edit can be a deletion
    (remove one letter), a transposition (swap adjacent letters), an alteration
    (change one letter to another) or an insertion (add a letter).
  * Check if word is known (ie. found in training file)
  * Suggest best correction to misspelled words
"]

use input;

const ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyz";

fn edits1(word: &str) -> Vec<String> {
    let mut v = Vec::new();
    let word_len = word.len();
    let mut splits = Vec::new();

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

fn edits2(e1s: &Vec<String>) -> Vec<String> {
    let mut v = Vec::new();
    for e1 in e1s {
        for e2 in edits1(&e1) {
            v.push(e2);
        }
    }
    v
}

fn known(words: &Vec<String>, word_library: &input::CountTable) -> Vec<(String, usize)> {
    let mut v = Vec::new();
    for word in words {
        match word_library.get(word) {
            Some(&freq) => v.push((word.to_owned(), freq)),
            None => continue
        }
    }
    v
}

pub fn correct(word: &str, word_library: &input::CountTable) -> String {
    let e1s = edits1(word);
    let mut candidates = known(&e1s, word_library);
    if candidates.len() <= 0 {
        candidates = known(&edits2(&e1s), word_library);
    }
    let mut best_word: String = "-".to_owned();
    let mut best_word_score: usize = 0;
    for pair in &candidates {
        if  pair.1 > best_word_score {
            best_word = pair.0.to_owned();
            best_word_score = pair.1;
        }
    }
    best_word
}

#[cfg(test)]
mod edits1_tests {

    use super::edits1;

    #[test]
    fn edits1_for_a() {
        let word = "a";
        let v = edits1(word);
        assert_eq!(78, v.len());
        let subset = vec!["b", "c", "d", "e", "f", "x", "y", "z", "ab", "ac",
                          "ad", "ae", "af", "ax", "ay", "az", "ab", "ac", "ad",
                          "ae", "af", "ax", "ay", "az"];
        for word in subset {
            assert!(v.contains(&word.to_owned()));
        }
    }

    #[test]
    fn edits1_for_e() {
        let word = "e";
        let v = edits1(word);
        assert_eq!(78, v.len());
        let subset = vec!["b", "c", "d", "f", "x", "y", "z", "eb", "ec", "ed",
                          "ef", "ex", "ey", "ez", "eb", "ec", "ed", "ee", "ef",
                          "ex", "ey", "ez"];
        for word in subset {
            assert!(v.contains(&word.to_owned()));
        }
    }

    #[test]
    fn edits1_for_aaaaa() {
        let word = "aaaaa";
        let v = edits1(word);
        assert_eq!(278, v.len());
        let subset = vec!["aaaa", "aaaaaa", "aaaab", "aacaa", "aaadaa",
                          "faaaaa", "axaaaa", "aaaaya", "azaaa"];
        for word in subset {
            assert!(v.contains(&word.to_owned()));
        }
    }

    #[test]
    fn edits1_for_abc() {
        let word = "abc";
        let v = edits1(word);
        assert_eq!(182, v.len());
    }

    #[test]
    fn edits1_for_abcde() {
        let word = "abcde";
        let v = edits1(word);
        assert_eq!(286, v.len());
        let subset = vec!["abcd", "abde", "cbcde", "abcze", "abcdef", "yabcde",
                          "bcde", "atcde", "bacde"];
        for word in subset {
            assert!(v.contains(&word.to_owned()));
        }
    }

    #[test]
    fn edits1_for_abcdefghijklmnopqrstuvwxyz() {
        let word = "abcdefghijklmnopqrstuvwxyz";
        let v = edits1(word);
        assert_eq!(1378, v.len());
        let subset = vec!["abcdefhijklmnopqrstuvwxyz",
                          "abcdefghikjlmnopqrstuvwxyz",
                          "abcdefghijklmnopqrstuvwxyza",
                          "abcdeaghijklmnopqrstuvwxyz"];
        for word in subset {
            assert!(v.contains(&word.to_owned()));
        }
    }

    #[test]
    fn edits1_for_hello() {
        let word = "hello";
        let v = edits1(word);
        assert_eq!(284, v.len());
        let subset = vec!["helloh", "yello", "hlelo", "helol", "ahello",
                          "helo", "jello"];
        for word in subset {
            assert!(v.contains(&word.to_owned()));
        }
    }

    #[test]
    fn edits1_for_something() {
        let word = "something";
        let v = edits1(word);
        assert_eq!(494, v.len());
        let subset = vec!["somthing", "somdthing", "aomething", "somehting"];
        for word in subset {
            assert!(v.contains(&word.to_owned()));
        }
    }

    #[test]
    fn edits1_for_anything() {
        let word = "anything";
        let v = edits1(word);
        assert_eq!(442, v.len());
        let subset = vec!["anzthing", "anyting", "antyhing", "manything"];
        for word in subset {
            assert!(v.contains(&word.to_owned()));
        }
    }

}

#[cfg(test)]
mod correct_tests {

    use input;
    use super::correct;

    #[test]
    fn test_no_suggestions_found() {
        let lib = fixture();
        assert_eq!(correct("asjkdghlaksjdghls", &lib), "-".to_owned());
    }

    #[test]
    fn test_chooses_more_frequent_alternative() {
        // ho should suggest who over how 
        // since who has freq=2 while how has freq=1
        let lib = fixture();
        assert_eq!(correct("ho", &lib), "who".to_owned());
    }

    #[test]
    fn test_correct_suggestion() {
        let lib = fixture();
        assert_eq!(correct("wherf", &lib), "where".to_owned());
    }

    fn fixture() -> input::CountTable {
        let mut h = input::CountTable::new();
        h.insert("two".to_owned(), 2);
        h.insert("three".to_owned(), 3);
        h.insert("why".to_owned(), 1);
        h.insert("where".to_owned(), 2);
        h.insert("when".to_owned(), 1);
        h.insert("how".to_owned(), 1);
        h.insert("who".to_owned(), 2);
        
        assert_eq!(None, h.get("none"));
        assert_eq!(Some(&2), h.get("two"));
        assert_eq!(Some(&3), h.get("three"));
        assert_eq!(7, h.len());
        h
    }

}
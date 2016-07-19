#[doc="
Input:
* Constructs a BK Tree using the words in the given training file. Words in the
  training file are handled based on the rules specified in the input module.
* The program then reads words from stdin. If the word was in the training
  file, it is assumed to be spelled correctly. If the word was not in the
  training file, the program attempts to find the word with the least
  Levenshtein distance between it and the input word. In the case of a tie, the
  word with the highest frequency in the training file is chosen. If no words
  are found within a Levenshtein distance of 2, the program is unable to make a
  suggestion.

Output:
* The program prints each word from stdin on a separate line.
* If the word was found in the training file, it is printed alone.
* If the word was not found in the training file and the program found a
  suggested correction, the suggested word is printed next to the input word.
* If the word was not found in the training file but the program did not find a
  suggested correction, a dash is printed next to the input word.

ex.
  bed
  qhair, chair
  sflj, -

Note that the behavior of the spelling corrector is slightly different from the
Norvig corrector because of the use of Levenshtein distance rather than edit
distance. In particular, transposing letters adds 2 to the Levenshtein
distance, whereas it only adds 1 to the edit distance. Thus, in the case where
the Norvig corrector suggests a correction of transposing two letters for an
edit distance of 1, the BK tree corrector might suggest a different correction
with a Levenshtein distance of 1. For example, if the misspelled word is
'wordl' and 'world' appears more frequently in the training file than 'word',
the Norvig corrector will suggest 'world' over 'word' because they are each
edit distance 1 and world is more frequent. On the other hand, the BK tree
corrector will suggest 'word' over 'world' because the Levenshtein distance
between 'wordl' and 'word' is 1 but the Levenshtein distance between 'wordl'
and 'world' is 2.
"]

use std::{env, fs};
use std::collections::HashMap;
use std::io::stdin;

mod bktree;
mod input;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("Pass in one argument--the name of the training file");
    }
    let f = fs::File::open(&args[1]).expect("Error opening the training file");

    let v: Vec<String> = input::read_input(f);
    let root = bktree::Node::new(v[0].to_string(), 0, HashMap::new());
    let mut bk = bktree::BKTree::new(root);
    for w in &v {
        bk.add(w);
    }

    let input = input::read_input(stdin());
    
    for word in &input {
        let best_suggestion = bk.search(word, &2);
        if best_suggestion == "-" {
            // No suggestions found
            println!("{}, -", word);
        } else if best_suggestion.len() == 0 {
            // Word spelled correctly
            println!("{}", word);
        } else {
            // Best suggestion
            println!("{}, {}", word, best_suggestion);
        }
    }
}

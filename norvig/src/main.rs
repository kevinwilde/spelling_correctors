// Kevin Wilde
// NETID: kjw731
// EECS 395

#[doc="
Input:
Inserts each word in the given training file into a hashmap, where the key is the word and
the value is the number of times the word has been encountered. Words in the training file 
are handled based on the rules specified in the input module.
The program then reads words from stdin. If the word was in the training file, it is assumed 
to be spelled correctly. If the word was not in the training file, the program attempts to 
find the word with the least edit distance between it and the input word. In the case 
of a tie, the word with the highest frequency in the training file is chosen. If no words are 
found within an edit distance of 2, the program is unable to make a suggestion.

Output:
The program prints each word from stdin on a separate line.
If the word was found in the training file, it is printed alone.
If the word was not found in the training file and the program found a suggested correction, the 
suggested word is printed next to the input word.
If the word was not found in the training file but the program did not find a suggested correction, 
a dash is printed next to the input word.
ex.
  bed
  qhair, chair
  sflj, -
"]

use std::{env, fs, io};

mod input;
mod edits;

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
            println!("{}, {}", word, edits::correct(word, &word_library));
        }
    }
}

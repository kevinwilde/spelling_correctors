// Kevin Wilde
// NETID: kjw731
// EECS 395

#[doc="

"]

use std::{env, fs};
use std::io::stdin;

mod node;
mod bktree;
mod input;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() != 2 {
        panic!("Pass in one argument--the name of the training file");
    }
    let f = fs::File::open(&args[1]).expect("Error opening the training file");

    let v: Vec<String> = input::read_input(f);
    let root = node::Node{word: v[0].to_owned(), freq: 0, children: std::collections::HashMap::new()};
    let mut bk = bktree::BKTree{ root: root };
    for w in &v {
        bk.add(w);
    }

    let input = input::read_input(stdin());
    for word in &input {
    	let best_suggestion = bk.search(word, &2);
        if best_suggestion == "-" { // No suggestions found
            println!("{}, -", word);
        } else if best_suggestion.len() == 0 { // Word spelled correctly
            println!("{}", word);
        } else {
            println!("{}, {}", word, best_suggestion);
        }
    }
}

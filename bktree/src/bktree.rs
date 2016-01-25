// Kevin Wilde
// NETID: kjw731
// EECS 395

#[doc="

"]

use std;
use node;

pub struct BKTree {
	pub root: node::Node
}

impl BKTree {
	
	pub fn add(&mut self, word: &str) {
		traverse_tree(&mut self.root, word);
	}

	pub fn search(&self, word: &str, d: &usize) -> &str {
		let mut v = Vec::new();
		recursive_search(&self.root, &mut v, word, d);

		let mut max_freq = 0;
		let mut best_word = "-";
		for node in &v {
			if word == &node.word {
				return "";
			}
			if node.freq > max_freq {
				max_freq = node.freq;
				best_word = &node.word;
			}
		}
		best_word
	}

}

fn traverse_tree(node: &mut node::Node, word: &str) {
	let dist = levenshtein_distance(&node.word, word);
	if dist == 0 {
		node.freq += 1;
		return;
	}
	if node.children.contains_key(&dist) {
		traverse_tree(node.children.get_mut(&dist).expect("Impossible"), word);
	} else {
		node.children.insert(dist, node::Node{ word: word.to_owned(), freq: 1, children: std::collections::HashMap::new() });
	}
}

fn recursive_search<'a>(node: &'a node::Node, v: &mut Vec<&'a node::Node>, word: &str, d: &usize) {
	let cur_dist = levenshtein_distance(&node.word, word);
	if cur_dist == 0 {
		v.clear();
		assert!(v.len() == 0);
		v.push(node);
		return;
	}
	let mut min_dist = 0;
	if cur_dist > *d { // Check that cur_dist > d first to prevent usize from underflowing
		min_dist = cur_dist - d;
	}
	let max_dist = cur_dist + d;

	if cur_dist <= *d {
		v.push(&node);
	}
	for key in node.children.keys() {
		if key >= &min_dist && key <= &max_dist {
			recursive_search(&node.children.get(key).expect("Impossible"), v, word, d);
		}
	}
}

pub fn levenshtein_distance(w1: &str, w2: &str) -> usize {
	let len1 = w1.len();
	let len2 = w2.len();

	if len1 == 0 {
		return len2;
	}
	if len2 == 0 {
		return len1;
	}

	let mut d: Vec<Vec<usize>> = Vec::new();

	for i in 0..(len1+1) {
		d.push(vec![i]);
		for _ in 1..(len2+1) {
			d[i].push(0);
		}
	}
	for i in 0..(len2+1) {
		d[0][i] = i;
	}

	for i in 1..(len1+1) {
		for j in 1..(len2+1) {
			let mut same = 1;
			if w1[(i-1)..i] == w2[(j-1)..j] {
				same = 0;
			}
			d[i][j] = std::cmp::min(std::cmp::min(d[i-1][j]+1, d[i][j-1]+1), d[i-1][j-1]+same);
		}
	}
	d[len1][len2]
}

#[cfg(test)]
mod levenshtein_distance_tests {

	use super::levenshtein_distance;

	#[test]
	fn test_hello_world() {
		assert_eq!(levenshtein_distance("hello", "world"), 4);
	}

	#[test]
	fn test_cat_hat() {
		assert_eq!(levenshtein_distance("cat", "hat"), 1);
	}

	#[test]
	fn test_book_back() {
		assert_eq!(levenshtein_distance("book", "back"), 2);
	}

	#[test]
	fn test_lion_lying() {
		assert_eq!(levenshtein_distance("lion", "lying"), 3);
	}

	#[test]
	fn test_astrology_astronomy() {
		assert_eq!(levenshtein_distance("astrology", "astronomy"), 2);
	}

	#[test]
	fn test_books_slack() {
		assert_eq!(levenshtein_distance("books", "slack"), 5);
	}

	#[test]
	fn test_computer_him() {
		assert_eq!(levenshtein_distance("computer", "him"), 7);
	}

	#[test]
	fn test_elephant_elegant() {
		assert_eq!(levenshtein_distance("elephant", "elegant"), 2);
	}

	#[test]
	fn test_onomatopoeia_hyperbole() {
		assert_eq!(levenshtein_distance("onomatopoeia", "hyperbole"), 10);
	}
	
}
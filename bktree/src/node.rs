// Kevin Wilde
// NETID: kjw731
// EECS 395

#[doc="
Node struct for use in BK Tree
  word: the word represented by this node
  freq: the frequency of this word in the training file
  children: hashmap of this node's children where the key is the Levenshtein 
  distance between the two nodes' words and the value is the child node.
"]

use std::collections::HashMap;

pub struct Node {
	pub word: String,
	pub freq: usize,
	pub children: HashMap<usize, Node>
}

// Kevin Wilde
// NETID: kjw731
// EECS 395

#[doc="

"]

use std::collections::HashMap;

pub struct Node {
	pub word: String,
	pub freq: usize,
	pub children: HashMap<usize, Node>
}
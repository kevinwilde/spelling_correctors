#[doc="
BKTree
In a BK tree, the nodes represent words and the edges are weighted with the 
Levenshtein distance between adjacent nodes' words.

root: the root node of the BK tree
"]

use std::cmp::min;
use std::collections::HashMap;

pub struct Node {
    word: String,
    freq: usize,
    children: HashMap<usize, Node>,
}

impl Node {
    pub fn new(word: String,
               freq: usize,
               children: HashMap<usize, Node>)
               -> Node {
        Node {
            word: word,
            freq: freq,
            children: children,
        }
    }
}

pub struct BKTree {
    root: Node,
}

impl BKTree {

    pub fn new(root: Node) -> BKTree {
        BKTree {
            root: root,
        }
    }
    
    pub fn add(&mut self, word: &str) {
        traverse_tree(&mut self.root, word);
    }

    pub fn search(&self, word: &str, d: &usize) -> &str {
        let mut v = Vec::new();
        recursive_search(&self.root, &mut v, word, d);

        if v.len() == 0 {
            return "-";
        } else {

            v.sort_by(|a, b|  {
                
                // Sort by distance
                if a.0 != b.0 {
                    a.0.cmp(&b.0)
                }

                // If equal distance, sort by word frequency 
                else {
                    b.1.freq.cmp(&a.1.freq)
                }
            });

            // if there is an exact match (ie. dist == 0)
            if v[0].0 == 0 {
                return "";
            }

            else {
                return &v[0].1.word;
            }
        }
    }

}

fn traverse_tree(node: &mut Node, word: &str) {
    let dist = levenshtein_distance(&node.word, word);
    if dist == 0 {
        node.freq += 1;
        return;
    }
    if node.children.contains_key(&dist) {
        traverse_tree(node.children.get_mut(&dist).expect("Impossible"), word);
    } else {
        let child = Node::new(word.to_string(), 1, HashMap::new());
        node.children.insert(dist, child);
    }
}

fn recursive_search<'a>(node: &'a Node, v: &mut Vec<(usize, &'a Node)>, word: &str, d: &usize) {
    let cur_dist = levenshtein_distance(&node.word, word);
    let mut min_dist = 0;

    // Check that cur_dist > d first to prevent usize from underflowing
    if cur_dist > *d {
        min_dist = cur_dist - d;
    }

    let max_dist = cur_dist + d;

    if cur_dist <= *d {
        v.push((cur_dist, &node));
    }

    for key in node.children.keys() {
        if key >= &min_dist && key <= &max_dist {
            recursive_search(&node.children.get(key).expect("Impossible"), v, word, d);
        }
    }
}

fn levenshtein_distance(w1: &str, w2: &str) -> usize {
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
            d[i][j] = min(min(d[i-1][j]+1, d[i][j-1]+1), d[i-1][j-1]+same);
        }
    }
    d[len1][len2]
}

#[cfg(test)]
mod bk_tree_tests {

    use std::collections::HashMap;
    use super::Node;
    use super::BKTree;

    #[test]
    fn test_no_need_to_correct_word() {
        let bk = small_fixture();
        assert_eq!(bk.search("who", &1), "");
    }

    #[test]
    fn test_no_suggestions_found() {
        let bk = small_fixture();
        assert_eq!(bk.search("asjkdghlaksjdghls", &1), "-");
    }

    #[test]
    fn test_chooses_more_frequent_alternative() {
        // ho should suggest who over how 
        // since who has freq=2 while how has freq=1
        let bk = small_fixture();
        assert_eq!(bk.search("ho", &1), "who");
    }

    #[test]
    fn test_correct_suggestion() {
        let bk = small_fixture();
        assert_eq!(bk.search("wherf", &1), "where");
    }

    fn small_fixture() -> BKTree {
        let root = Node::new("what".to_string(), 0, HashMap::new());
        let mut bk = BKTree::new(root);
        bk.add("why");
        bk.add("where");
        bk.add("where");
        bk.add("when");
        bk.add("how");
        bk.add("who");
        bk.add("who");
        bk
    }

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
    fn test_world_wordl() {
        assert_eq!(levenshtein_distance("world", "wordl"), 2);
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

//! Implementation of BK-Trees with strings in mind

use std::{
    collections::VecDeque,
    convert::TryInto,
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

use regex::Regex;

/// Read each line of a text file into a vector
/// Used to read dictionaries into memory
fn read_lines(path: impl AsRef<Path>) -> Vec<String> {
    BufReader::new(File::open(path).expect("Could not open file!"))
        .lines()
        .map(|x| x.expect("Could not read line: {x}"))
        .collect()
}

#[derive(Debug)]
/// A single node in a BKTree.
/// BKNodes store their string value, as well as a vector of arcs weighted by the metric used
struct BKNode {
    /// The string stored in the node
    val: String,
    /// The node's children. Each edge is weighted by their distance
    children: Vec<(BKNode, usize)>,
}

impl BKNode {
    #[inline]
    fn new(s: &str) -> Self {
        BKNode {
            val: s.to_string(),
            children: Vec::new(),
        }
    }
}

/// A struct for a BKTree data structure
/// Each node stores a word, as the tree is intended to be used for Spell Checking and fuzzy string search
pub struct BKTree {
    /// The root node of the tree
    root: Option<Box<BKNode>>,
    /// The distance metric used by the tree
    metric: fn(&str, &str) -> usize,
    /// The number of nodes in the tree
    node_count: usize,
    /// An optional regex used by the tree to ignore certain strings (eg: numbers)
    ignore_re: Option<Regex>,
    /// A boolean indicating whether or not to ignore casing (treat 'heLlO' the same as 'hello')
    ignore_case: bool,
}

impl BKTree {
    /// Construct a BKTree given its distance metric
    /// # Arguments
    /// * 'f' - a function of the form: fn(&str, &str) -> usize to be used as the distance function
    /// * 'ignore_case' - a boolean indicating whether or not the tree will ignore the case of characters
    /// eg: "hELlO" will be treated the same as "hello"
    pub fn new(f: fn(&str, &str) -> usize, ignore_case: bool) -> Self {
        BKTree {
            root: None,
            metric: f,
            node_count: 0,
            ignore_re: None,
            ignore_case: ignore_case,
        }
    }
    #[inline]
    /// Set the tree's ignore regex
    /// any incoming strings which match the regex are ignored and no effort is made to suggest corrections
    pub fn ignore(&mut self, re: &str) {
        self.ignore_re = Some(Regex::new(re).expect("Invalid regex!"));
    }
    /// Helper function
    /// Given the path to a txt file of words, read all of them into the tree
    pub fn read_dict<P: AsRef<Path>>(&mut self, corpus: P) {
        match self.ignore_case {
            false => {
                for word in read_lines(corpus).iter() {
                    self.add_word(word.as_str());
                }
            }
            true => {
                for word in read_lines(corpus).iter() {
                    self.add_word(word.to_lowercase().as_str());
                }
            }
        }
    }
    /// Insert a vector of strings into the tree
    pub fn read_vec(&mut self, corpus: Vec<&str>) {
        match self.ignore_case {
            false => {
                for word in corpus.iter() {
                    self.add_word(word);
                }
            }
            true => {
                for word in corpus.iter() {
                    self.add_word(word.to_lowercase().as_str());
                }
            }
        }
    }
    /// Adds a string into the tree
    pub fn add_word(&mut self, word: &str) {
        self.node_count += 1;
        match self.root {
            None => {
                self.root = Some(Box::new(BKNode::new(word)));
            }
            Some(ref mut root) => {
                let mut curr = &mut **root;

                loop {
                    let dist = (self.metric)(curr.val.as_str(), word);
                    if dist == 0 {
                        return;
                    }

                    let x = curr.children.iter().position(|(_, k)| dist == *k);
                    match x {
                        None => {
                            curr.children.push((BKNode::new(word), dist));
                            return;
                        }
                        Some(k) => {
                            let (ref mut node, _) = curr.children[k];
                            curr = node;
                        }
                    }
                }
            }
        }
    }
    /// Returns the best correction given a string and the max edit distance
    pub fn query(&self, word: &str, k: usize) -> Option<&String> {
        // check if the incoming string matches the ignore regex
        match self.ignore_re {
            None => (),
            Some(ref k) => {
                if k.is_match(word) {
                    return None;
                }
            }
        }

        match self.root {
            None => None,
            Some(ref root) => {
                let mut S: VecDeque<&BKNode> = VecDeque::new();
                S.push_back(root);

                let mut best_node: Option<&BKNode> = None;
                let mut best_k = usize::MAX;

                while let Some(u) = S.pop_front() {
                    let mut k_u: usize = (self.metric)(&u.val, word);

                    if k_u < best_k {
                        best_node = Some(&u);
                        best_k = k_u;
                    }

                    for v in u.children.iter() {
                        let (v_node, k_uv) = v;
                        // Cutoff criterion
                        if (*k_uv as isize - k_u as isize).abs() < best_k.try_into().unwrap() {
                            S.push_back(v_node);
                        }
                    }
                }

                Some(&best_node.unwrap().val)
            }
        }
    }
    /// Return a vector of corrections and their corresponding distances given a word/ string as well as the max distance
    pub fn corrections(&self, word: &str, k: usize) -> Vec<(&str, usize)> {
        match self.root {
            None => Vec::new(),
            Some(ref root) => {
                let mut S: VecDeque<&BKNode> = VecDeque::new();
                S.push_back(root);

                let mut corrections: Vec<(&str, usize)> = Vec::new();

                while let Some(u) = S.pop_front() {
                    let dist = (self.metric)(&u.val, word);

                    if dist <= k {
                        corrections.push((u.val.as_str(), dist));
                    }

                    for v in u.children.iter().filter(|(_, d)| (*d as isize - dist as isize).abs() <= k as isize) {
                        let (v_node, _) = v;
                        // Cutoff criterion
                        S.push_back(v_node);
                    }
                }

                corrections
            }
        }
    }
}
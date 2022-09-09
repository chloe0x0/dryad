//! Implementation of BK-Trees with strings in mind
//! Metrics should be taken from metric.rs, but there is nothing stopping the curious from experimenting with custom string metrics

// TODO
// Use multithreading to speed up indexing and search

use std::{
    fs::File, 
    io::{self, BufReader, BufRead}, 
    path::Path, 
    collections::{HashMap, VecDeque},
    time::{Instant, Duration}, 
    thread,
};

fn read_lines(path: impl AsRef<Path>) -> Vec<String> {
    BufReader::new(File::open(path).expect("Could not open file!"))
        .lines()
        .map(|x| x.expect("Could not read line {x}"))
        .collect()
}

// BKNodes store their string value, as well as a vector of arcs weighted by the metric used
#[derive(Debug)]
struct BKNode {
    val: String,
    children: Vec<(BKNode, isize)>, 
}

impl BKNode {
    fn new(s: &str) -> Self {
        BKNode { val: s.to_string(), children: Vec::new() }
    }
}

// It would probably be useful to have an O(1) way of returning immediatley if a word is known to be in the dictionary
// Can store a Hashset of all the Strings....
// Sounds scary, lots of space O(k) for k unique words in the corpus,
// Bloom filters should be killer for this 

pub struct BKTree {
    root: Option<Box<BKNode>>,
    metric: fn(&str, &str) -> isize,
    node_count: usize,
}

impl BKTree {
    pub fn new(f: fn(&str, &str) -> isize) -> Self {
        BKTree { root: None, metric: f, node_count: 0 }
    }
    pub fn read_corpus(&mut self, corpus: impl AsRef<Path>) {
        let xs = read_lines(corpus);

        for word in xs.iter() {
            self.add_word(word.as_str());
        }
    }
    pub fn read_vec(&mut self, corpus: Vec<&str>) {
        for word in corpus.iter() {
            self.add_word(word);
        }
    }
    pub fn add_word(&mut self, word: &str) {
        self.node_count += 1;
        match self.root {
            None => {
                self.root = Some(Box::new(BKNode::new(word)));
            } 
            Some(ref mut root) => {
                let mut curr = &mut **root;

                loop {
                    let dist: isize = (self.metric)(curr.val.as_str(), word);
                    if dist == 0 {
                        return;
                    }
                    
                    let x = curr.children.iter().position(|(_, k)| dist == *k);
                    match x {
                        None => {
                            curr.children.push(
                                (BKNode::new(word), dist)
                            );
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
    pub fn spell_check_word(&self, word: &str, k: isize) -> Option<&String> {
        match self.root {
            None => None,
            Some(ref root) => {
                let mut S: VecDeque<&BKNode> = VecDeque::new();
                S.push_back(root);

                let mut best_node: Option<&BKNode> = None;
                let mut best_k: isize = isize::MAX;

                while let Some(u) = S.pop_front() {
                    let k_u = (self.metric)(&u.val, word);
                    if k_u < best_k {
                        best_node = Some(&u);
                        best_k = k_u;
                    }

                    for v in u.children.iter() {
                        let v_node = &(v.0);
                        let k_uv = (self.metric)(&u.val, &v_node.val);
                        // Cutoff criterion
                        if (k_uv - k_u).abs() < best_k {
                            S.push_back(v_node);
                        }
                    }
                }
                
                if &best_node.unwrap().val.as_str() == &word { None } else { Some(&best_node.unwrap().val) }
            }
        }
    }
    pub fn spell_check(&self, text: &str) -> Vec<(String, String)> {
        text.split(" ")
            .filter(|x| !self.spell_check_word(&x, 1).is_none())
            .map(|x| (x.to_string(), self.spell_check_word(x, 1).unwrap().to_string()))
            .collect()
    }
}

// Takes about 7 seconds to index a dictionary of 466k words, 6 if the cache is warmed up (maybe)
// ^^ above is on an AMD Ryzen 3700U (base speed 2.3GHz)
// reading the file into a vector is NOT the bottleneck
// Perhaps there is a more optimal way of constructing the BK-Tree such that the time to insert is minimized for a given metric

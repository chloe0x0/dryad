//! Implementation of BK-Trees with strings in mind
//! Metrics should be taken from metric.rs, but there is nothing stopping the curious from experimenting with custom string metrics

mod metric;
use metric::*;

use std::{
    fs::File, 
    io::{self, BufReader, BufRead}, 
    path::Path, 
    collections::{HashMap},
    time::{Instant, Duration}
};

fn read_lines(path: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(path).expect("Could not open file!");
    let buffer = BufReader::new(file);

    buffer.lines()
        .map(|x| x.expect("Could not read line {x}"))
        .collect()
}

#[derive(Debug)]
struct BKNode {
    val: String,
    children: Vec<(BKNode, usize)>,
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
    metric: fn(&str, &str) -> usize,
    node_count: usize,
}

impl BKTree {
    pub fn empty(f: fn(&str, &str) -> usize) -> Self {
        BKTree { root: None, metric: f, node_count: 0 }
    }
    pub fn read_corpus(&mut self, corpus: impl AsRef<Path>) {
        let xs = read_lines(corpus);

        for word in xs.iter() {
            self.add_word(&word[..]);
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
                    let dist: usize = (self.metric)(&curr.val[..], word);
                    if dist == 0 as usize {
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
}

fn main() {
    let start = Instant::now();
    let mut t = BKTree::empty(lev);
    t.read_corpus("../dicts/words.txt");
    println!("Time taken to index dictionary of {} words: {} seconds", t.node_count, start.elapsed().as_secs());
}
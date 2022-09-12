//! Implementation of BK-Trees with strings in mind
//! Metrics should be taken from metric.rs, but there is nothing stopping the curious from experimenting with custom string metrics

// TODO
// Use multithreading to speed up indexing and search

use std::{
    collections::VecDeque,
    convert::TryInto,
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

use regex::Regex;

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
    children: Vec<(BKNode, usize)>,
}

impl BKNode {
    fn new(s: &str) -> Self {
        BKNode {
            val: s.to_string(),
            children: Vec::new(),
        }
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
    ignore_re: Option<Regex>,
}

impl BKTree {
    pub fn new(f: fn(&str, &str) -> usize) -> Self {
        BKTree {
            root: None,
            metric: f,
            node_count: 0,
            ignore_re: None,
        }
    }
    pub fn ignore(&mut self, re: &str) {
        self.ignore_re = Some(Regex::new(re).unwrap());
    }
    pub fn read_dict(&mut self, corpus: impl AsRef<Path>) {
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
    pub fn spell_check_word(&self, word: &str, k: usize, ingore_case: bool) -> Option<&String> {
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
                    let mut k_u: usize = 0;
                    if ingore_case {
                        k_u = (self.metric)(&u.val.to_lowercase(), &word.to_lowercase());
                    } else {
                        k_u = (self.metric)(&u.val, word)
                    }
                    
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

                if ingore_case {
                    if &best_node.unwrap().val.as_str().to_lowercase() == &word.to_lowercase() {
                        return None;
                    }
                } else {
                    if &best_node.unwrap().val.as_str() == &word {
                        return None;
                    }
                }

                Some(&best_node.unwrap().val)
            }
        }
    }
    #[inline(always)]
    pub fn spell_check(&self, text: &str, ingore_case: bool) -> Vec<(String, String)> {
        text.split(" ")
            .filter(|x| !self.spell_check_word(&x, 1, ingore_case).is_none())
            .map(|x| {
                (
                    x.to_string(),
                    self.spell_check_word(x, 1, ingore_case)
                        .unwrap()
                        .to_string(),
                )
            })
            .collect()
    }
}
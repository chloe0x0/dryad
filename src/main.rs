mod metric;
use metric::{ham, lev};

mod bktree;
use bktree::*;

use std::io;

// unit tests for the string metrics
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_lev() {
        assert_eq!(lev("sitting", "kitten"), 3);
        assert_eq!(lev("Truck", "Track"), 1);
        // lev(a, b) == lev(b, a)
        assert_eq!(lev("kitten", "sitting"), lev("sitting", "kitten"));
    }
    #[test]
    fn test_ham() {
        assert_eq!(ham("01", "11"), 1);
        assert_eq!(ham("01", "10"), 2);
        assert_eq!(ham("101", "000"), 2);
    }
}

fn main() {
    let mut t: BKTree = BKTree::new(lev);
    t.read_corpus("../dicts/MIT.txt");

    let mut text = String::new();
    io::stdin().read_line(&mut text).expect("Could not read line");

    let corrections = t.spell_check(text.as_str());

    for word in text.split(" ") {
        match corrections.iter().find(|(x, _)| x == word) {
            None => print!("{} ", word),
            Some(c) => print!("++{}++", c.1)
        }
    }   
}

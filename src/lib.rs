//!
//! A spell checking and fuzzy string search library in Rust using BK-Trees
//! Metrics should be taken from metric.rs, but there is nothing stopping the curious from experimenting with custom string metrics
//! Dryad will NOT verify that your custom function is indeed a valid distance metric
//! 
//! ```rust
//! use dryad::{bktree::BKTree, metric::lev};
//! let mut tree = BKTree::new(lev, true);
//! tree.read_vec(vec![
//!   "book", "books", "cake", "boo", "boon", "cook", "cake", "cape", "cart",
//! ]);
//!
//! // [book, boo, boon]
//! println!("{:?}", tree.corrections("bo", 2));
//!    
//!
//! // [cake, cape, cart]
//! println!("{:?}", tree.corrections("ca", 2));
//! ```
//! 

pub mod bktree;
pub mod metric;

pub use bktree::*;
pub use metric::*;

#[cfg(test)]
mod tests {
    use crate::{lev, BKTree};

    #[test]
    fn lev_test() {
        let mut t = BKTree::new(lev, true);
        t.read_vec(vec![
            "book", "books", "cake", "boo", "boon", "cook", "cake", "cape", "cart",
        ]);

        let corrections = t.corrections("bo", 2);

        let (strs, _): (Vec<&str>, Vec<usize>) = corrections.into_iter().unzip();
        assert_eq!(strs, ["book", "boo", "boon"]);
    }
}

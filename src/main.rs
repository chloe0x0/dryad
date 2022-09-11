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
    // build a system similar to git command spell check with Dryad
    let mut git = BKTree::new(lev);
    git.read_vec(vec!["push", "pull", "branch", "commit"]);
    let cmd = "branhc";

    match git.spell_check_word(cmd, 1) {
        None => println!("Executed {}", cmd),
        Some(k) => println!("git: '{}' is not a git command. See 'git --help'.\n\nThe most similar command is\n\t{}", cmd, k)
    }

    println!("{:?}", git.spell_check(cmd));
}

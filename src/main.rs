mod metric;
use metric::lev;

mod bktree;
use bktree::*;

use std::io;

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

mod metric;
use metric::lev;

mod bktree;
use bktree::*;

use std::io;

fn main() {
    // build a system similar to git command spell check with Dryad
    let mut git = BKTree::new(lev);
    git.read_vec(vec!["push", "pull", "branch", "commit"]);
    git.ignore(r"[0-9]+");
    let cmd = "comiitt 2147190 128590 85 brach";


    println!("{:?}", git.spell_check(cmd));
}

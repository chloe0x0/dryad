mod metric;
use metric::lev;

mod bktree;
use bktree::*;

use std::io;

fn main() {
    let mut tree = BKTree::new(lev);
    tree.read_dict("../dicts/MIT.txt");
    tree.ignore(r"[0-9]+");

    let mut input = String::from("Hello wold 0");
   
    let corrections = tree.spell_check(&input, true);

    println!("{}", input);

    for word in input.split(" ") {
        match corrections.iter().find(|(x, y)| &x==&word) {
            None => print!("{} ", word),
            Some(ref k) => print!("+{}+ ", k.1)
        }
    }
}
use dryad::{bktree::BKTree, metric::lev};

fn main() {
    let mut tree = BKTree::new(lev);
    tree.read_dict("../dicts/MIT.txt");
    tree.ignore(r"[0-9]+");

    let input = String::from("hEll world!");

    let corrections = tree.spell_check(&input, 1, true);

    println!("{}", input);

    for word in input.split(" ") {
        match corrections.iter().find(|(x, y)| &x == &word) {
            None => print!("{} ", word),
            Some(ref k) => print!("+{}+ ", k.1),
        }
    }
}

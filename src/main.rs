use dryad::*;

fn main() {
    let mut tree = BKTree::new(lev, true);
    tree.read_vec(vec![
        "book", "books", "cake", "boo", "boon", "cook", "cake", "cape", "cart",
    ]);

    // [book, boo, boon]
    println!("{:?}", tree.corrections("bo", 2));

    // [cake, cape, cart]
    println!("{:?}", tree.corrections("ca", 2));
}

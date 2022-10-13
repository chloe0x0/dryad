# Dryad 🌴
A fast spell-checker and fuzzy string searcher in Rust 

## How does it work?
BK-Trees 

### Creating a BK-Tree
```rust
use dryad::*;

fn main() {
    // The BK-Tree constructor only needs a metric function
    // and a boolean indicating whether or not to ignore case
    // can use any fn of the form: fn(&str, &str) -> usize
    let mut tree = BKTree::new(lev, true);    
}
```


## A working example

```rust
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
```

### Ignoring strings that match a regex
it may be the case that you do not want Dryad to suggest correcting digits. eg: changing 1 => "a"

It is simple to specify an ignore regex. Any substrings which match this regex will simply be ignored

```rust
let mut tree = BKTree::new(lev, true);
tree.ignore(r"[0-9]+");

// The tree will now ignore digits and wont suggest corrections to them
```

## TODO
1. Better English dictionary

## MISC
1. Look into publishing as a crate on crates.io
2. There seems to be an absence of good resources on BK-Trees, maybe write a markdown file explaining them
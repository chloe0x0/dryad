# Dryad 🌴
A fast spell-checker and fuzzy string searcher in Rust 

## How does it work?
BK-Trees 

### Creating a BK-Tree
```rust
use bktree::*;     // Actual BK-Tree struct
use metric::lev;   // A fast edit distance implementation 

fn main() {
    // The BK-Tree constructor only needs a metric function
    // can use any fn of the form: fn(&str, &str) -> usize
    let mut tree = BKTree::new(lev);    
}
```

### Create a git command checker
if one gives git a command it does not recognize it will reccomend similarly spelled commands. 
eg: 
```console
> git comit -m "Spelling is hard!"
git: 'comit' is not a git command. See 'git --help'.

The most similar command is
        commit
```

Lets build a similar system with Dryad
```rust
use bktree::*;
use metric::lev;

fn main() {
    let mut git = BKTree::new(lev);
    // read in the commands we want to suggest corrections for
    git.read_vec(vec!["push", "pull", "branch", "commit"]);
    let cmd = "comit";

    match git.spell_check_word(cmd, 1) {
        None => println!("Executed {}", cmd),
        Some(k) => println!("git: '{}' is not a git command. See 'git --help'.\n\nThe most similar command is\n\t{}", cmd, k)
    }
}
```

## TODO
1. Implement a function to get the top N corrections rather than just 1
2. Better English dictionary
3. Benchmark
4. Fuzzy String Search API
5. Add a function on the BK-Tree structure which would allow it to ignore any strings which match a particular regex

## MISC
1. Look into publishing as a crate on crates.io
2. There seems to be an absence of good resources on BK-Trees, maybe write a markdown file explaining them
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

### Ignoring substrings that match a regex
it may be the case that you do not want Dryad to suggest correcting digits. eg: changing 1 => "a"

It is simple to specify an ignore regex. Any substrings which match this regex will simply be ignored
```rust
use bktree::*;
use metric::lev;

fn main() {
    let mut tree = BKTree::new(lev);
    // read_dict will split a txt file by lines and add each line to the tree
    // the dicts subfolder contains various dictionaries
    // MIT.txt is sourced from: https://www.mit.edu/~ecprice/wordlist.10000
    tree.read_dict("../dicts/MIT.txt");
    tree.ignore(r"[0-9]+");

    let input = String::from("Hello 215 wold 0");
   
    // generate a Vec<(String, String)> in which the first element is the original string
    // and the second element is the suggested correction
    // BKTree::spell_check takes the input string and a boolean indicating whether or not to ignore case
    let corrections = tree.spell_check(&input, true);

    println!("{}", input);

    for word in input.split(" ") {
        match corrections.iter().find(|(x, y)| &x==&word) {
            None => print!("{} ", word),
            Some(ref k) => print!("+{}+ ", k.1)
        }
    }
}
```

will output
```console
> cargo run --release
Hello 215 wold 0
Hello 215 +world+ 0 
```

if the ignore regex is not specified to ignore numbers
```console
> cargo run --release
Hello 215 wold 0
Hello +a+ +world+ +a+ 
``` 

if the spell_check function does not ignore case
```console
> cargo run --release
Hello 215 wold 0
+hello+ 215 +world+ 0
```
the above happens because "Hello" is not in the loaded dictionary, but "hello" is
this is the major motivation for including the case ignore parameter

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

## MISC
1. Look into publishing as a crate on crates.io
2. There seems to be an absence of good resources on BK-Trees, maybe write a markdown file explaining them

// String metric implementations

// Using the Wagner-Fischer algorithm to compute the levenshtien distance of two strings
pub fn lev(s0: &str, s1: &str) -> usize {
    let s0_chars: Vec<char> = s0.chars().collect();
    let s1_chars: Vec<char> = s1.chars().collect();    

    // get lengths of strings
    let len_0: usize = s0_chars.len();
    let len_1: usize = s1_chars.len();

    let mut row: Vec<usize> = vec![0 as usize; len_1 + 1];

    let mut d0: usize = 0; 

    // initialization
    for i in 0..=len_1 {
        row[i] = i;
    }

    for i in 0..len_0 {
        let mut e: usize = i + 1;

        for j in 0..len_1 {
            let c: usize = (s0_chars[i] != s1_chars[j]) as usize;
            d0 = std::cmp::min (
                std::cmp::min(
                    row[j + 1] + 1, 
                    e + 1
                ),
                row[j] + c
            ); 

            row[j] = e;
            e = d0;
        }

        row[len_1] = d0;
    }

    return row[len_1];
}


fn main() {
    assert_eq!(lev("sitting", "kitten"), 3);
}


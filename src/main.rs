mod metric;
use metric::{ham, lev};

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
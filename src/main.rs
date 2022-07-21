#[cfg(test)]
mod tests {
    #[path = r"D:\Wattson\src\Metric.rs"]
    mod Metric;
    use Metric::{lev, ham};
    #[test]
    fn test_lev() {
        assert_eq!(lev("sitting", "kitten"), 3);
        assert_eq!(lev("Truck", "Track"), 1);
        assert_eq!(lev("kitten", "sitting"), lev("sitting", "kitten"));
    }
    #[test]
    fn test_ham() {
        assert_eq!(ham("01", "11"), 1);
        assert_eq!(ham("01", "10"), 2);
        assert_eq!(ham("101", "000"), 2);
    }
}

fn main() {
    println!("Hello, world!");
}

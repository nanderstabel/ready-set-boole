use rsb::gray_code;

#[allow(dead_code)]
fn main() {
    gray_code(4294967295);
}

#[cfg(test)]
mod gray_code {
    use super::*;

    #[test]
    fn assert_equal() {
        assert_eq!(gray_code(0), 0);
        assert_eq!(gray_code(1), 1);
        assert_eq!(gray_code(2), 3);
        assert_eq!(gray_code(3), 2);
        assert_eq!(gray_code(4), 6);
        assert_eq!(gray_code(5), 7);
        assert_eq!(gray_code(6), 5);
        assert_eq!(gray_code(7), 4);
        assert_eq!(gray_code(8), 12);
        assert_eq!(gray_code(4294967295), 2147483648);
    }
}

use rsb::adder;

#[allow(dead_code)]
fn main() {
    adder(0, 0);
}

#[cfg(test)]
mod adder {
    use super::*;

    #[test]
    fn assert_equal() {
        assert_eq!(adder(0, 0), 0);
        assert_eq!(adder(1, 0), 1);
        assert_eq!(adder(0, 1), 1);
        assert_eq!(adder(31, 31), 62);
        assert_eq!(adder(42, 42), 84);
        assert_eq!(adder(4294967295, 0), 4294967295);
    }

    #[test]
    fn overflow() {
        assert_eq!(adder(4294967295, 1), 0);
        assert_eq!(adder(4294967295, 2), 1);
        assert_eq!(adder(4294967295, 4294967295), 4294967294);
    }

    #[test]
    fn eval_sheet() {
        assert_eq!(adder(0, 0), 0);
        assert_eq!(adder(1, 0), 1);
        assert_eq!(adder(0, 1), 1);
        assert_eq!(adder(1, 1), 2);
        assert_eq!(adder(1, 2), 3);
        assert_eq!(adder(2, 2), 4);
    }
}

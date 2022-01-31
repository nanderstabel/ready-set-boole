use rsb::multiplier;

#[allow(dead_code)]
fn main() {
    multiplier(0, 0);
}

#[cfg(test)]
mod multiplier {
    use super::*;

    #[test]
    fn assert_equal() {
        assert_eq!(multiplier(0, 0), 0);
        assert_eq!(multiplier(1, 0), 0);
        assert_eq!(multiplier(0, 1), 0);
        assert_eq!(multiplier(1, 1), 1);
        assert_eq!(multiplier(42, 42), 1764);
        assert_eq!(multiplier(4294967295, 1), 4294967295);
    }

    #[test]
    fn overflow() {
        assert_eq!(multiplier(4294967295, 4294967295), 1);
    }

    #[test]
    fn eval_sheet() {
        assert_eq!(multiplier(0, 0), 0);
        assert_eq!(multiplier(1, 0), 0);
        assert_eq!(multiplier(0, 1), 0);
        assert_eq!(multiplier(1, 1), 1);
        assert_eq!(multiplier(1, 2), 2);
        assert_eq!(multiplier(2, 2), 4);
    }
}

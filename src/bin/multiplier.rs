mod adder;
use adder::adder;

fn multiplier(a: u32, b: u32) -> u32 {
    let (mut a, mut b, mut res) = (a, b, 0);
    while b > 0 {
        if b & 1 != 0 {
            res = adder(res, a);
        }
        a = a << 1;
        b = b >> 1;
    }
    res
}

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
}

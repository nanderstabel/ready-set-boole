fn adder(a: u32, b: u32) -> u32 {
    let (mut a, mut b) = (a, b);
    while b != 0 {
        let carry = a & b;
        a = a ^ b;
        b = carry << 1;
    }
    a
}

fn main() {
    adder(0, 0);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn assert_equal() {
        assert_eq!(adder(0, 0), 0);
        assert_eq!(adder(1, 0), 1);
        assert_eq!(adder(0, 1), 1);
        assert_eq!(adder(35, 42), 77);
        assert_eq!(adder(42, 42), 84);
        assert_eq!(adder(4294967295, 0), 4294967295);
    }

    #[test]
    fn overflow() {
        assert_eq!(adder(4294967295, 1), 0);
        assert_eq!(adder(4294967295, 2), 1);
        assert_eq!(adder(4294967295, 4294967295), 4294967294);
    }
}

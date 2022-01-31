use rsb::map;
use rsb::reverse_map;

#[allow(dead_code)]
fn main() {
    println!("{:#?}", reverse_map(0.011154345006485085));
}

#[cfg(test)]
mod curve {
    use super::*;

    #[test]
    fn assert_equal() {
        assert_eq!(reverse_map(0.0), (0, 0));
        assert_eq!(reverse_map(0.000000010244548323155508), (2, 6));
        assert_eq!(reverse_map(0.009790322279974428), (364, 6323));
        assert_eq!(reverse_map(0.019531563627424548), (12345, 1));
        assert_eq!(reverse_map(0.0020089102913646286), (314, 2526));
        assert_eq!(reverse_map(1.0), (u16::MAX, u16::MAX));
    }

    #[test]
    fn inverse() {
        assert_eq!(reverse_map(map(0, 0)), (0, 0));
        assert_eq!(reverse_map(map(2, 6)), (2, 6));
        assert_eq!(reverse_map(map(364, 6323)), (364, 6323));
        assert_eq!(reverse_map(map(12345, 1)), (12345, 1));
        assert_eq!(reverse_map(map(314, 2526)), (314, 2526));
        assert_eq!(reverse_map(map(u16::MAX, u16::MAX)), (u16::MAX, u16::MAX));
    }
}

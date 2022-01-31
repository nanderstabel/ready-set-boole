use rsb::map;

#[allow(dead_code)]
fn main() {
    println!("{:#?}", map(u16::MAX, u16::MAX));
}

#[cfg(test)]
mod curve {
    use super::*;

    #[test]
    fn assert_equal() {
        assert_eq!(map(0, 0), 0.0);
        assert_eq!(map(2, 6), 0.000000010244548323155508);
        assert_eq!(map(364, 6323), 0.009790322279974428);
        assert_eq!(map(12345, 1), 0.019531563627424548);
        assert_eq!(map(314, 2526), 0.0020089102913646286);
        assert_eq!(map(u16::MAX, u16::MAX), 1.0);
    }
}

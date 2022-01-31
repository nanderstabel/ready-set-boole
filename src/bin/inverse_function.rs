fn reverse_map(n: f64) -> (u16, u16) {
    let res = (n * u32::MAX as f64) as u32;
    let (mut x, mut y) = (0u16, 0u16);
    for b in (0..32).step_by(2) {
        if res & (1 << b) != 0 {
            x |= 1 << (b / 2)
        }
        if res & (1 << (b + 1)) != 0 {
            y |= 1 << (b / 2)
        }
    }
    (x, y)
}

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
}

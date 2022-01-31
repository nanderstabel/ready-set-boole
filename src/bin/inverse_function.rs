fn reverse_map(n: f64) -> (u16, u16) {
    let res = (n * u16::MAX as f64) as u32;
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

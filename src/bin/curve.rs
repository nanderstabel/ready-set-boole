fn map(x: u16, y: u16) -> f64 {
    let mut res = 0u32;
    for b in (0..32).step_by(2) {
        if x & (1 << (b / 2)) != 0 {
            res |= 1 << b;
        }
        if y & (1 << (b / 2)) != 0 {
            res |= 1 << (b + 1);
        }
    }
    res as f64 / u16::MAX as f64
}

#[allow(dead_code)]
fn main() {
    println!("{:#?}", map(13, 27));
}

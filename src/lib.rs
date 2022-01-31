pub mod group;
pub mod kmap;
pub mod parser;
pub mod permutationlist;
pub mod truthtable;

pub fn adder(a: u32, b: u32) -> u32 {
    let (mut a, mut b) = (a, b);
    while b != 0 {
        let carry = (a & b) << 1;
        a = a ^ b;
        b = carry;
    }
    a
}

pub fn multiplier(a: u32, b: u32) -> u32 {
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

pub fn gray_code(n: u32) -> u32 {
    n ^ (n >> 1)
}

pub fn map(x: u16, y: u16) -> f64 {
    let mut res = 0u32;
    for b in (0..32).step_by(2) {
        if x & (1 << (b / 2)) != 0 {
            res |= 1 << b;
        }
        if y & (1 << (b / 2)) != 0 {
            res |= 1 << (b + 1);
        }
    }
    res as f64 / u32::MAX as f64
}

pub fn reverse_map(n: f64) -> (u16, u16) {
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

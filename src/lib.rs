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

#![allow(dead_code)]

fn insertion(n: u32, m: u32, i: u8, j: u8) -> u32 {
    let mask = !(!((!0b0) << (j - i)) << i);

    (n & mask) | (m << i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion() {
        assert_eq!(0b10001001100, insertion(0b10000000000, 0b10011, 2, 6));
    }
}

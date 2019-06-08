#![allow(dead_code)]

fn insertion(n: u32, m: u32, i: u8, j: u8) -> u32 {
    let mask = !(!((!0b0) << (j - i)) << i);

    (n & mask) | (m << i)
}

fn flip_bit(n: u32) -> usize {
    let mut n = n;
    let mut seqs = vec![];
    let mut this_seq = vec![];
    let mut this_ones_len = 0;
    let mut seen_gap = true;

    while n != 0 {
        let bit = n & 1;
        println!("bit is {}", bit);
        if bit == 1 {
            if seen_gap {
                this_ones_len = 1;
            } else {
                this_ones_len += 1;
            }
            seen_gap = false;
        } else {
            if seen_gap {
                println!("pushing seq {:?}", this_seq);
                seqs.push(this_seq);
                this_seq = vec![];
            } else {
                seen_gap = true;
                this_seq.push(this_ones_len);
            }
        }

        n = n >> 1;
    }

    this_seq.push(this_ones_len);
    seqs.push(this_seq);

    println!("{:?}", seqs);

    seqs.iter().fold(0, |acc, xs| {
        acc.max(xs.windows(2).fold(0, |acc, xs| acc.max(xs[0] + xs[1] + 1)))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_insertion() {
        assert_eq!(0b10001001100, insertion(0b10000000000, 0b10011, 2, 6));
    }

    #[test]
    fn test_flip_bit() {
        assert_eq!(8, flip_bit(1775));
        assert_eq!(6, flip_bit(0b111011001101101));
    }
}

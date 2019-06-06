#![allow(dead_code)]
use std::collections::{HashMap, HashSet};

fn is_unique(s: &str) -> bool {
    let mut seen = HashSet::new();

    for c in s.chars() {
        seen.insert(c);
    }

    seen.len() == s.len()
}

fn is_unique_no_structure(s: &str) -> bool {
    for (i, c1) in s.chars().enumerate() {
        for (_, c2) in s[i + 1..].chars().enumerate() {
            if c1 == c2 {
                return false;
            }
        }
    }

    true
}

fn check_permutations(s1: &str, s2: &str) -> bool {
    let mut counts1 = HashMap::new();
    let mut counts2 = HashMap::new();

    for c in s1.chars() {
        counts1.entry(c).and_modify(|e| *e += 1).or_insert(1);
    }

    for c in s2.chars() {
        counts2.entry(c).and_modify(|e| *e += 1).or_insert(1);
    }

    if counts1.len() != counts2.len() {
        return false;
    }

    for (k1, v1) in counts1.iter() {
        if let Some(v2) = counts2.get(k1) {
            if v1 != v2 {
                return false;
            }
        } else {
            return false;
        }
    }

    return true;
}

fn urlify(s: &mut [char]) {
    let mut cursor = s.len() - s.iter().rev().take_while(|c| *c == &' ').count() - 1;

    let mut i = s.len() - 1;

    while i > 0 {
        if s[cursor] == ' ' {
            s[i] = '0';
            s[i - 1] = '2';
            s[i - 2] = '%';

            i -= 3;
        } else {
            s[i] = s[cursor];
            i -= 1;
        }

        cursor -= 1;
    }
}

fn is_palindrome(s: &str, offset: usize) -> bool {
    let forwards = s.chars().chain(s.chars()).skip(offset);
    let backwards = s.chars().chain(s.chars()).rev().skip(s.len() - offset);

    forwards
        .zip(backwards)
        .take(s.len() / 2)
        .all(|(x, y)| x == y)
}

fn palindrome_perm(s: &str) -> bool {
    let mut chars = HashMap::new();

    for c in s.chars().filter(|c| *c != ' ') {
        chars.entry(c).and_modify(|e| *e += 1).or_insert(1);
    }

    let num_odd = chars.values().filter(|v| *v % 2 == 1).count();

    if s.chars().filter(|c| *c != ' ').count() % 2 == 1 {
        num_odd == 1
    } else {
        num_odd == 0
    }
}

fn one_away(s1: &str, s2: &str) -> bool {
    let mut used_edit = false;

    let cs1: Vec<char> = s1.chars().collect();
    let cs2: Vec<char> = s2.chars().collect();

    let mut i1 = 0;
    let mut i2 = 0;

    while i1 < cs1.len() {
        let c1 = cs1[i1];

        if i2 >= cs2.len() {
            if used_edit {
                return false;
            }

            return i1 + 1 >= cs1.len();
        } else {
            let c2 = cs2[i2];

            if c1 == c2 {
                i1 += 1;
                i2 += 1;
            } else {
                if i1 + 1 < cs1.len() && c2 == cs1[i1 + 1] {
                    // delete this one

                    if used_edit {
                        return false;
                    }

                    used_edit = true;
                    i1 += 1;
                } else if i1 + 1 < cs1.len() && i2 + 1 < cs2.len() && cs1[i1 + 1] == cs2[i2 + 1] {
                    // replace

                    if used_edit {
                        return false;
                    }

                    used_edit = true;
                    i1 += 1;
                    i2 += 1;
                } else {
                    // insert one

                    if used_edit {
                        return false;
                    }

                    used_edit = true;
                    i1 += 1;
                    i2 += 1;
                }
            }
        }
    }

    true
}

struct Acc {
    output: String,
    letter: char,
    times: usize,
}

fn string_compression(s: &str) -> String {
    let cs: Vec<char> = s.chars().collect();

    let mut acc: Acc = cs.iter().skip(1).fold(
        Acc {
            output: String::new(),
            letter: cs[0],
            times: 1,
        },
        |mut acc, &c| {
            if c == acc.letter {
                acc.times += 1;
            } else {
                acc.output.push_str(&format!("{}{}", acc.letter, acc.times));
                acc.times = 1;
                acc.letter = c;
            }

            acc
        },
    );

    acc.output.push_str(&format!("{}{}", acc.letter, acc.times));
    if acc.output.len() < s.len() {
        acc.output
    } else {
        s.to_string()
    }
}

fn rotate_matrix(image: &mut Vec<Vec<u8>>) {
    let layers = image.len() / 2;
    let n = image.len();

    for layer in 0..layers {
        println!("Layer {}", layer);
        for i in 0..(n - (2 * layer)) - 1 {
            println!("Swapping at i: {}", i);
            let tmp = image[layer][layer + i];
            image[layer][layer + i] = image[n - layer - i - 1][layer];
            image[n - layer - i - 1][layer] = image[n - layer - 1][n - layer - i - 1];
            image[n - layer - 1][n - layer - i - 1] = image[layer + i][n - layer - 1];
            image[layer + i][n - layer - 1] = tmp;
        }
    }
}

fn zero_matrix(m: &mut Vec<Vec<u8>>) {
    let mut rows = HashSet::new();
    let mut cols = HashSet::new();

    for x in 0..m.len() {
        for y in 0..m[x].len() {
            if m[x][y] == 0 {
                rows.insert(x);
                cols.insert(y);
            }
        }
    }

    for row in rows {
        for j in 0..m[row].len() {
            m[row][j] = 0;
        }
    }

    for col in cols {
        for i in 0..m.len() {
            m[i][col] = 0;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_is_unique() {
        assert_eq!(true, is_unique("abcdefghi"));
        assert_eq!(false, is_unique("aabbcdef"));
        assert_eq!(false, is_unique("abdcea"));
    }

    #[test]
    fn test_is_unique_no_structure() {
        assert_eq!(true, is_unique_no_structure("abcdefghi"));
        assert_eq!(false, is_unique_no_structure("aabbcdef"));
        assert_eq!(false, is_unique_no_structure("abdcea"));
    }

    #[test]
    fn test_check_permutations() {
        assert_eq!(true, check_permutations("abcd", "cbad"));
        assert_eq!(true, check_permutations("abcda", "cbaad"));
        assert_eq!(false, check_permutations("abcdax", "cbaad"));
        assert_eq!(false, check_permutations("abcdax", "xcbad"));
    }

    #[test]
    fn test_urlify() {
        let mut s: Vec<char> = "Mr John Smith    ".chars().collect();
        urlify(&mut s);

        let expected: Vec<char> = "Mr%20John%20Smith".chars().collect();

        assert_eq!(expected, s);
    }

    #[test]
    fn test_is_palindrome() {
        assert_eq!(true, is_palindrome("abcba", 0));
        assert_eq!(true, is_palindrome("abccba", 0));
        assert_eq!(true, is_palindrome("abba", 0));
        assert_eq!(true, is_palindrome("a", 0));
        assert_eq!(false, is_palindrome("abca", 0));
        assert_eq!(false, is_palindrome("abca", 0));

        assert_eq!(true, is_palindrome("bcbaa", 4));
    }

    #[test]
    fn test_palindrome_perm() {
        assert_eq!(true, palindrome_perm("tact coa"))
    }

    #[test]
    fn test_one_away() {
        assert_eq!(true, one_away("pale", "ple"));
        assert_eq!(true, one_away("pales", "pale"));
        assert_eq!(true, one_away("pale", "bale"));
        assert_eq!(false, one_away("pale", "bake"));
    }

    #[test]
    fn test_string_compression() {
        assert_eq!("a2b1c5a3".to_string(), string_compression("aabcccccaaa"));
        assert_eq!("abc".to_string(), string_compression("abc"));
        assert_eq!("abbc".to_string(), string_compression("abbc"));
    }

    #[test]
    fn test_rotate_matrix() {
        let mut input = vec![vec![1, 2], vec![3, 4]];

        rotate_matrix(&mut input);

        assert_eq!(vec![vec![3, 1], vec![4, 2]], input);

        let mut input = vec![
            vec![1, 2, 3, 4],
            vec![2, 3, 4, 1],
            vec![3, 4, 1, 2],
            vec![4, 1, 2, 3],
        ];

        rotate_matrix(&mut input);

        assert_eq!(
            vec![
                vec![4, 3, 2, 1],
                vec![1, 4, 3, 2],
                vec![2, 1, 4, 3],
                vec![3, 2, 1, 4]
            ],
            input
        );
    }

    #[test]
    fn test_zero_matrix() {
        let mut input = vec![
            vec![1, 1, 1, 1],
            vec![1, 1, 0, 1],
            vec![1, 1, 1, 1],
            vec![0, 1, 1, 1],
        ];

        zero_matrix(&mut input);

        assert_eq!(
            vec![
                vec![0, 1, 0, 1],
                vec![0, 0, 0, 0],
                vec![0, 1, 0, 1],
                vec![0, 0, 0, 0]
            ],
            input
        );
    }
}

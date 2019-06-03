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
}

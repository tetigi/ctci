#![allow(dead_code)]

fn unit(x: u32) -> Option<String> {
    match x % 10 {
        0 => None,
        1 => Some("One"),
        2 => Some("Two"),
        3 => Some("Three"),
        4 => Some("Four"),
        5 => Some("Five"),
        6 => Some("Six"),
        7 => Some("Seven"),
        8 => Some("Eight"),
        9 => Some("Nine"),
        _ => panic!(),
    }
    .map(|s| s.to_string())
}

fn tens(x: u32) -> Option<String> {
    let tens = match (x % 100) / 10 {
        0 => None,
        1 => Some("Ten"),
        2 => Some("Twenty"),
        3 => Some("Thirty"),
        4 => Some("Forty"),
        5 => Some("Fifty"),
        6 => Some("Sixty"),
        7 => Some("Seventy"),
        8 => Some("Eightty"),
        9 => Some("Ninety"),
        _ => panic!("{} -> {}", x, x % 100),
    }
    .map(|s| s.to_string());

    let units = unit(x);

    if let Some(tens) = tens {
        if let Some(units) = units {
            if x % 100 == 1 {
                match x % 10 {
                    0 => Some("Ten"),
                    1 => Some("Eleven"),
                    2 => Some("Twelve"),
                    3 => Some("Thirteen"),
                    4 => Some("Fourteen"),
                    5 => Some("Fifteen"),
                    6 => Some("Sixteen"),
                    7 => Some("Seventeen"),
                    8 => Some("Eighteen"),
                    9 => Some("Nineteen"),
                    _ => panic!(),
                }
                .map(|s| s.to_string())
            } else {
                Some(format!("{} {}", tens, units))
            }
        } else {
            Some(tens)
        }
    } else {
        units
    }
}

fn hundreds(x: u32) -> Option<String> {
    let hundreds = match (x % 1000) / 100 {
        0 => None,
        _ => unit((x % 1000) / 100).map(|x| format!("{} Hundred", x)),
    };

    let tens = tens(x);

    if let Some(hundreds) = hundreds {
        if let Some(tens) = tens {
            Some(format!("{} and {}", hundreds, tens))
        } else {
            Some(hundreds)
        }
    } else {
        tens
    }
}

fn thousands(x: u32) -> Option<String> {
    let thousands = match x % 100_000 {
        0 => None,
        _ => hundreds((x % 100_000) / 1000).map(|x| format!("{} Thousand", x)),
    };

    let rest = hundreds(x % 1000);

    if let Some(thousands) = thousands {
        if let Some(rest) = rest {
            Some(format!("{}, {}", thousands, rest))
        } else {
            Some(thousands)
        }
    } else {
        rest
    }
}

fn english_int(x: u32) -> String {
    thousands(x).unwrap()
}

use std::iter;

fn multiply(a: i32, b: i32) -> i32 {
    iter::repeat(a).take(b as usize).fold(0, |acc, x| acc + x)
}

//fn subtract(a: i32, b: i32) -> i32 {
//    unimplemented!()
//}
//
//fn divide(a: i32, b: i32) -> i32 {
//    unimplemented!()
//}

use std::collections::HashMap;

fn living_people(people: Vec<(u32, u32)>) -> u32 {
    let mut living_years = HashMap::new();

    for (birth, death) in people {
        for y in birth..=death {
            living_years.entry(y).and_modify(|e| *e += 1).or_insert(1);
        }
    }

    living_years
        .iter()
        .max_by_key(|(&_, &v)| v)
        .unwrap()
        .0
        .clone()
}

use std::collections::HashSet;

fn diving_board(shorter: u32, longer: u32, k: u32) -> Vec<u32> {
    // if no 'same', then can have 0s kl, 1s, (k-1)l, 2s, (k-2)l etc.
    // == k possible choices
    // but if ml == ns for some m, n then that erases an extra option
    // 0 < m, n < k
    // find lcm of s and l, then there are k / lcm erasures
    let mut lengths = HashSet::new();

    for i in 0..=k {
        lengths.insert(shorter * i + longer * (k - i));
    }

    lengths.into_iter().collect()
}

#[derive(Debug, PartialEq)]
struct Line {
    m: f32,
    c: f32,
}

impl Line {
    fn construct(p1: &(f32, f32), p2: &(f32, f32)) -> Line {
        let m = (p2.1 - p1.1) / (p2.0 - p1.0);
        let c = p1.1 - m * p1.0;

        Line { m, c }
    }

    fn contains(&self, p: &(f32, f32)) -> bool {
        p.1 == self.m * p.0 + self.c
    }
}

fn best_line(points: Vec<(f32, f32)>) -> Line {
    let mut best_line = Line::construct(&points[0], &points[1]);
    let mut most_fit = 0;

    for (i, p1) in points.iter().enumerate() {
        for p2 in points[i + 1..].iter() {
            let line = Line::construct(p1, p2);

            let fit = points[i + 2..].iter().filter(|p| line.contains(p)).count();
            if fit > most_fit {
                most_fit = fit;
                best_line = line;
            }
        }
    }

    best_line
}

#[derive(Debug, PartialEq)]
struct Score {
    hits: usize,
    p_hits: usize,
}

fn master_mind(guess: &str, solution: &str) -> Score {
    let misses: Vec<(char, char)> = guess
        .chars()
        .zip(solution.chars())
        .filter(|(g, s)| g != s)
        .collect();
    let hits = guess.len() - misses.len();
    let (guesses, actual) = misses.iter().fold(
        (HashMap::new(), HashMap::new()),
        |(mut guesses, mut actual), (g, s)| {
            guesses.entry(g).and_modify(|e| *e += 1).or_insert(1);
            actual.entry(s).and_modify(|e| *e += 1).or_insert(1);
            (guesses, actual)
        },
    );

    let p_hits = guesses
        .iter()
        .map(|(colour, count)| actual.get(colour).unwrap_or(&0).min(count))
        .sum();

    Score { hits, p_hits }
}

fn sub_sort(xs: Vec<usize>) -> (usize, usize) {
    let front = xs.windows(2).take_while(|xs| xs[0] <= xs[1]).count() + 1;

    let back = xs.windows(2).rev().take_while(|xs| xs[0] <= xs[1]).count() + 1;

    let max_front = xs[0..(xs.len() - back)].iter().max().unwrap();
    let min_back = xs[front..xs.len()].iter().min().unwrap();

    let m = xs.iter().take_while(|x| *x <= min_back).count();
    let n = xs.len() - xs.iter().rev().take_while(|x| *x >= max_front).count() - 1;

    (m, n)
}

fn contiguous_sequence(xs: Vec<isize>) -> isize {
    let mut max_val = *xs.last().unwrap();
    let mut current_seq = vec![max_val];
    let mut current_val = max_val;

    for v in xs.iter().rev().skip(1) {
        if current_val + *v > *v {
            current_val += v;
            current_seq.push(*v);

            if current_val > max_val {
                max_val = current_val;
            }
        } else {
            current_seq = vec![*v];
            current_val = *v;
        }
    }

    max_val
}

fn pattern_match(value: &str, pattern: &str) -> bool {
    let mut counts: HashMap<char, usize> = vec![('a', 0), ('b', 0)].into_iter().collect();

    for c in pattern.chars() {
        counts.entry(c).and_modify(|e| *e += 1).or_insert(1);
    }

    if counts[&'a'] == 0 {
        let test: String = value
            .chars()
            .take(value.len() / counts[&'b'])
            .cycle()
            .take(value.len())
            .collect();
        return value == test;
    } else if counts[&'b'] == 0 {
        let test: String = value
            .chars()
            .take(value.len() / counts[&'a'])
            .cycle()
            .take(value.len())
            .collect();
        return value == test;
    }

    let a_multiples = value.len() / counts[&'a'];

    'outer: for a_size in 1..=a_multiples {
        let b_size = (value.len() - (a_size * counts[&'a'])) / counts[&'b'];

        let mut a_candidate = None;
        let mut b_candidate = None;
        let mut cursor = 0;

        for p in pattern.chars() {
            match p {
                'a' => {
                    if let Some(a) = a_candidate {
                        if a != &value[cursor..cursor + a_size] {
                            continue 'outer;
                        }
                    } else {
                        a_candidate = Some(&value[cursor..cursor + a_size]);
                    }

                    cursor += a_size;
                }
                'b' => {
                    if let Some(b) = b_candidate {
                        if b != &value[cursor..cursor + b_size] {
                            continue 'outer;
                        }
                    } else {
                        b_candidate = Some(&value[cursor..cursor + b_size]);
                    }

                    cursor += b_size;
                }
                _ => panic!(),
            }
        }

        return true;
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_english_int() {
        assert_eq!(
            "One Thousand, Two Hundred and Thirty Four",
            english_int(1234)
        );
    }

    #[test]
    fn test_living_people() {
        let people = vec![(1900, 1990), (1971, 1980), (1979, 2010)];

        let result = living_people(people);
        assert_eq!(true, result == 1980 || result == 1979);
    }

    #[test]
    fn test_best_line() {
        let points = vec![(-1.0, -1.0), (0.0, 0.0), (1.0, 1.0), (1.0, -1.0)];

        assert_eq!(Line { m: 1.0, c: 0.0 }, best_line(points));
    }

    #[test]
    fn test_master_mind() {
        assert_eq!(Score { hits: 1, p_hits: 1 }, master_mind("GGRR", "RGBY"));
    }

    #[test]
    fn test_sub_sort() {
        assert_eq!(
            (3, 9),
            sub_sort(vec![1, 2, 4, 7, 10, 11, 7, 12, 6, 7, 16, 18, 19])
        );

        assert_eq!((3, 4), sub_sort(vec![1, 2, 4, 8, 7, 16, 18, 19]));
    }

    #[test]
    fn test_contiguous_subsequence() {
        assert_eq!(5, contiguous_sequence(vec![2, -8, 3, -2, 4, -10]));
    }

    #[test]
    fn test_pattern_matching() {
        assert_eq!(true, pattern_match("catcatgocatgo", "aabab"));
        assert_eq!(true, pattern_match("catcatgocatgo", "a"));
        assert_eq!(true, pattern_match("catcatgocatgo", "ab"));
        assert_eq!(true, pattern_match("catcatgocatgo", "b"));
    }
}

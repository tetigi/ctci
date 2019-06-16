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
            living_years.entry(y).and_modify(|e| *e += 1).or_insert(0);
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
        let people = vec![(1900, 1990), (1991, 1980), (1979, 2010)];

        assert_eq!(1979, living_people(people));
    }
}

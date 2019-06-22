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

fn expand_pond_size(
    ponds: &Vec<Vec<usize>>,
    seen: &mut HashSet<(usize, usize)>,
    i: usize,
    j: usize,
) -> usize {
    let width = ponds.len() as isize;
    let height = ponds[0].len() as isize;

    seen.insert((i, j));

    let neighbours: Vec<(usize, usize)> = vec![
        (1, 0),
        (0, 1),
        (0, -1),
        (-1, 0),
        (1, 1),
        (-1, -1),
        (1, -1),
        (-1, 1),
    ]
    .into_iter()
    .map(|(x, y)| (x + (i as isize), y + (j as isize)))
    .filter(|(x, y)| {
        *x >= 0
            && *x < width
            && *y >= 0
            && *y < height
            && ponds[*x as usize][*y as usize] == 0
            && !seen.contains(&(*x as usize, *y as usize))
    })
    .map(|(x, y)| (x as usize, y as usize))
    .collect();

    let mut pond_size = 1;
    for (ni, nj) in neighbours {
        pond_size += expand_pond_size(ponds, seen, ni, nj);
    }

    pond_size
}

fn pond_sizes(ponds: Vec<Vec<usize>>) -> Vec<usize> {
    let mut seen = HashSet::new();
    let mut pond_sizes = vec![];

    for (i, row) in ponds.iter().enumerate() {
        for (j, height) in row.iter().enumerate() {
            if seen.contains(&(i, j)) || *height != 0 {
                continue;
            } else {
                let pond = expand_pond_size(&ponds, &mut seen, i, j);
                pond_sizes.push(pond);
            }
        }
    }

    pond_sizes
}

fn sum_swap(xs: Vec<isize>, ys: Vec<isize>) -> (isize, isize) {
    let sum_xs: isize = xs.iter().cloned().sum();
    let sum_ys: isize = ys.iter().cloned().sum();
    let sum_diff = sum_xs - sum_ys;

    let x_vals: HashSet<isize> = xs.iter().cloned().collect();

    ys.iter()
        .cloned()
        .filter_map(|y| {
            let target_x = (sum_diff / 2) + y;
            if x_vals.contains(&target_x) {
                Some((target_x, y))
            } else {
                None
            }
        })
        .next()
        .unwrap()
}

#[derive(Debug, Copy, Clone)]
enum GridColour {
    WHITE,
    BLACK,
}

impl GridColour {
    fn flip(&self) -> GridColour {
        match self {
            GridColour::WHITE => GridColour::BLACK,
            GridColour::BLACK => GridColour::WHITE,
        }
    }
}

struct InfiniteGrid {
    active_points: HashMap<(isize, isize), GridColour>,
    max_y: isize,
    min_y: isize,
    max_x: isize,
    min_x: isize,
}

impl InfiniteGrid {
    fn print(&self) {
        for y in self.min_y..=self.max_y {
            for x in self.min_x..=self.max_x {
                let colour = self
                    .active_points
                    .get(&(x, y))
                    .unwrap_or(&GridColour::WHITE);
                let output = match colour {
                    GridColour::WHITE => "0",
                    GridColour::BLACK => "1",
                };

                print!("{} ", output);
            }
            println!();
        }
    }

    fn flip(&mut self, x: isize, y: isize) {
        self.active_points
            .entry((x, y))
            .and_modify(|e| *e = e.flip())
            .or_insert(GridColour::BLACK);
        self.max_y = self.max_y.max(y);
        self.min_y = self.min_y.min(y);
        self.max_x = self.max_x.max(x);
        self.min_x = self.min_x.min(x);
    }

    fn get_colour(&mut self, x: isize, y: isize) -> &GridColour {
        self.max_y = self.max_y.max(y);
        self.min_y = self.min_y.min(y);
        self.max_x = self.max_x.max(x);
        self.min_x = self.min_x.min(x);

        self.active_points
            .entry((x, y))
            .or_insert(GridColour::WHITE)
    }
}

impl Default for InfiniteGrid {
    fn default() -> Self {
        let mut active_points = HashMap::new();
        active_points.insert((0, 0), GridColour::WHITE);

        InfiniteGrid {
            active_points,
            max_y: 0,
            min_y: 0,
            max_x: 0,
            min_x: 0,
        }
    }
}

#[derive(Debug)]
enum Dir {
    RIGHT,
    DOWN,
    LEFT,
    UP,
}

impl Dir {
    fn rotate_cw(&self) -> Dir {
        match self {
            Dir::RIGHT => Dir::DOWN,
            Dir::DOWN => Dir::LEFT,
            Dir::LEFT => Dir::UP,
            Dir::UP => Dir::RIGHT,
        }
    }

    fn rotate_ccw(&self) -> Dir {
        match self {
            Dir::RIGHT => Dir::UP,
            Dir::DOWN => Dir::RIGHT,
            Dir::LEFT => Dir::DOWN,
            Dir::UP => Dir::LEFT,
        }
    }

    fn pos_diff(&self) -> (isize, isize) {
        match self {
            Dir::RIGHT => (1, 0),
            Dir::DOWN => (0, 1),
            Dir::LEFT => (-1, 0),
            Dir::UP => (0, -1),
        }
    }
}

fn print_langtons_ant(k: usize) {
    let mut pos = (0, 0);
    let mut grid: InfiniteGrid = Default::default();
    let mut direction = Dir::RIGHT;

    for _ in 0..k {
        let colour = grid.get_colour(pos.0, pos.1);

        match colour {
            GridColour::WHITE => {
                direction = direction.rotate_cw();
            }
            GridColour::BLACK => {
                direction = direction.rotate_ccw();
            }
        }

        grid.flip(pos.0, pos.1);

        let (dx, dy) = direction.pos_diff();
        pos = (pos.0 + dx, pos.1 + dy);
    }

    grid.print();
}

use std::rc::Rc;

struct Node<'a, T> {
    data: &'a T,
    next: Option<Rc<Node<'a, T>>>,
    prev: Option<Rc<Node<'a, T>>>,
}

impl<'a, T> Node<'a, T> {
    fn new<'b>(data: &'b T) -> Node<'b, T> {
        Node {
            data,
            next: None,
            prev: None,
        }
    }

    fn unlink(&mut self) {
        if let Some(next) = &mut self.next {
            if let Some(prev) = &mut self.prev {
                if let Some(n) = Rc::get_mut(next) {
                    n.next = Some(Rc::clone(prev));
                }
                if let Some(p) = Rc::get_mut(prev) {
                    p.prev = Some(Rc::clone(next));
                }
            } else if let Some(v) = Rc::get_mut(next) {
                v.prev = None;
            }
        } else if let Some(prev) = &mut self.prev {
            if let Some(v) = Rc::get_mut(prev) {
                v.next = None;
            }
        }
    }
}

/* I tried, okay??

use std::hash::Hash;

struct LRUCache<'a, K, V> {
    first_last: Option<(Rc<Node<'a, K>>, Rc<Node<'a, K>>)>,
    cache: HashMap<&'a K, V>,
    max_size: usize,
}

impl<'a, K: Hash + Eq, V> LRUCache<'a, K, V> {
    fn init<'b>(max_size: usize) -> LRUCache<'b, K, V> {
        LRUCache {
            first_last: None,
            cache: HashMap::with_capacity(max_size),
            max_size,
        }
    }

    fn get(&mut self, key: K) -> Option<V> {
        unimplemented!()
    }

    fn cache(&mut self, key: &'a K, value: V) {
        if self.cache.len() < self.max_size {
            self.cache.insert(key, value);
        } else {
            if let Some((first, last)) = &mut self.first_last {
                if self.cache.len() == 1 {
                    self.first_last = None;
                } else {
                    let last_but_one = if let Some(rc) = &last.prev {
                        Rc::clone(rc)
                    } else {
                        panic!()
                    };

                    self.first_last = Some((Rc::clone(&first), last_but_one));
                }

                self.cache.insert(key, value);
            } else {
                self.cache.insert(key, value);
                let node = Rc::new(Node::new(key));
                let node_copy = Rc::clone(&node);
                self.first_last = Some((node, node_copy));
            }
        }
    }
}

*/

// / -> * -> + -> -
fn calculator(expr: &str) -> f64 {
    fn pluses(expr: &str) -> f64 {
        expr.split('+')
            .map(|x| times(x))
            .fold(0.0, |res, x| res + x)
    }

    fn times(expr: &str) -> f64 {
        expr.split('*')
            .map(|x| divides(x))
            .fold(1.0, |res, x| res * x)
    }

    fn divides(expr: &str) -> f64 {
        let ops: Vec<f64> = expr.split('/').map(|x| x.parse().unwrap()).collect();

        if ops.len() == 1 {
            ops[0]
        } else if ops.len() > 2 {
            panic!("Uh oh! {}", expr);
        } else {
            ops[0] / ops[1]
        }
    }

    let minuses: Vec<f64> = expr.split('-').map(|x| pluses(x)).collect();
    println!("{:?}", minuses);
    let mut res = minuses[0];

    for ops in minuses.iter().skip(1) {
        res -= ops;
    }

    res
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

    #[test]
    fn test_pond_sizes() {
        assert_eq!(
            vec![2, 4, 1],
            pond_sizes(vec![
                vec![0, 2, 1, 0],
                vec![0, 1, 0, 1],
                vec![1, 1, 0, 1],
                vec![0, 1, 0, 1],
            ])
        )
    }

    #[test]
    fn test_sum_swap() {
        assert_eq!((1, 3), sum_swap(vec![4, 1, 2, 1, 1, 2], vec![3, 6, 3, 3]));
    }

    #[test]
    fn test_langtons_ant() {
        print_langtons_ant(200);
    }

    #[test]
    fn test_calculator() {
        assert_eq!(23.5, calculator("2*3+5/6*3+15")); //6 + 5/18 + 15 == 21 + 15/6 = 2 + 3/6 = 23.5
        assert_eq!(-27.0, calculator("2-6-7*8/2+5")); //6 + 5/18 + 15 == 21 + 15/6 = 2 + 3/6 = 23.5
    }
}

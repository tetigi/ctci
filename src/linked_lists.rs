use std::cell::Cell;
use std::collections::HashSet;
use std::fmt;
use std::rc::Rc;

struct Node {
    next: Option<Box<Cell<Node>>>,
    value: usize,
}

impl Node {
    fn from(xs: Vec<usize>) -> Node {
        (*xs.iter()
            .rev()
            .fold(None, |acc, x| {
                Some(Box::new(Cell::new(Node {
                    next: acc,
                    value: *x,
                })))
            })
            .unwrap())
        .into_inner()
    }
}

//impl fmt::Debug for Node {
//    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//        if let Some(next) = &self.next {
//            write!(f, "{} -> {:?}", self.value, (*next).next)
//        } else {
//            write!(f, "{}", self.value)
//        }
//    }
//}

//fn remove_duplicates(list: Node) {
//    let mut seen = HashSet::new();
//
//    seen.insert(list.value);
//
//    let mut current = list;
//    while true {
//        let mut last = current;
//    }
//}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn test_remove_duplicates() {
//        let ll = Node::from(vec![1, 2, 3, 4]);
//        println!("{:?}", ll);
//
//        remove_duplicates(ll);
//        panic!()
//    }
//}

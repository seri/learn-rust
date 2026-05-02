#[derive(Debug)]
enum BinaryTree<T> {
    Empty,
    Node {
        value: T,
        left: Box<BinaryTree<T>>,
        right: Box<BinaryTree<T>>,
    },
}

impl<T: PartialOrd> BinaryTree<T> {
    fn add(&mut self, elem: T) {
        match self {
            BinaryTree::Empty => {
                *self = BinaryTree::Node {
                    value: elem,
                    left: Box::new(BinaryTree::Empty),
                    right: Box::new(BinaryTree::Empty),
                }
            }
            BinaryTree::Node { value, left, right } => {
                if elem < *value {
                    left.add(elem)
                } else if elem > *value {
                    right.add(elem)
                }
            }
        }
    }

    fn contains(&self, elem: T) -> bool {
        match self {
            BinaryTree::Empty => false,
            BinaryTree::Node { value, left, right } => {
                if elem < *value {
                    left.contains(elem)
                } else if elem > *value {
                    right.contains(elem)
                } else {
                    true
                }
            }
        }
    }
}

struct InorderIter<'a, T> {
    stack: Vec<&'a BinaryTree<T>>,
}

impl<'a, T> IntoIterator for &'a BinaryTree<T> {
    type Item = &'a T;
    type IntoIter = InorderIter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        InorderIter::new(self)
    }
}

impl<'a, T> InorderIter<'a, T> {
    fn new(tree: &'a BinaryTree<T>) -> Self {
        let mut iter = Self { stack: Vec::new() };
        iter.push_left(tree);
        iter
    }

    fn push_left(&mut self, mut tree: &'a BinaryTree<T>) {
        while let BinaryTree::Node {
            value: _,
            left,
            right: _,
        } = tree
        {
            self.stack.push(tree);
            tree = left;
        }
    }
}

impl<'a, T> Iterator for InorderIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.stack.pop() {
            Some(BinaryTree::Node { value, right, .. }) => {
                self.push_left(right);
                Some(value)
            }
            _ => None,
        }
    }
}

fn main() {
    let mut tree: BinaryTree<isize> = BinaryTree::Empty;
    tree.add(2);
    tree.add(1);
    tree.add(3);
    println!("{:?}", tree);

    for elem in &tree {
        println!("{:}", elem);
    }

    let mut tree2: BinaryTree<&str> = BinaryTree::Empty;
    tree2.add("bbb");
    tree2.add("ccc");
    tree2.add("aaa");
    println!("{:?}", tree2);
    println!("{:?}", tree2.contains("ddd"));
    println!("{:?}", tree2.contains("aaa"));

    for elem in &tree2 {
        println!("{:}", elem);
    }
}

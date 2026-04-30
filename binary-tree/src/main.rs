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
}

fn main() {
    let mut tree: BinaryTree<isize> = BinaryTree::Empty;
    tree.add(2);
    tree.add(1);
    tree.add(3);
    println!("{:?}", tree);
}

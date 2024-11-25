use std::cell::RefCell;
use std::rc::Rc;

// Define a type for a shared reference to a node
type TreeNodeRef<T> = Option<Rc<RefCell<TreeNode<T>>>>;

// Define the structure for a tree node
#[derive(Debug)]
struct TreeNode<T> {
    value: T,
    left: TreeNodeRef<T>,
    right: TreeNodeRef<T>,
}

impl<T> TreeNode<T> {
    // Create a new node
    fn new(value: T) -> TreeNodeRef<T> {
        Some(Rc::new(RefCell::new(TreeNode {
            value,
            left: None,
            right: None,
        })))
    }
}

// Binary Tree implementation
#[derive(Debug)]
struct BinaryTree<T> {
    root: TreeNodeRef<T>,
}

impl<T> BinaryTree<T> {
    // Create an empty binary tree
    fn new() -> Self {
        BinaryTree { root: None }
    }

    // Insert a value into the tree
    fn insert(&mut self, value: T)
    where
        T: Ord,
    {
        self.root = Self::insert_into(self.root.take(), value);
    }

    fn insert_into(node: TreeNodeRef<T>, value: T) -> TreeNodeRef<T>
    where
        T: Ord,
    {
        match node {
            Some(n) => {
                if value < n.borrow().value {
                    let left = n.borrow_mut().left.take();
                    n.borrow_mut().left = Self::insert_into(left, value);
                } else {
                    let right = n.borrow_mut().right.take();
                    n.borrow_mut().right = Self::insert_into(right, value);
                }
                Some(n)
            }
            None => TreeNode::new(value),
        }
    }

    // In-order traversal of the tree
    fn in_order_traversal(&self) {
        Self::in_order(&self.root);
    }

    fn in_order(node: &TreeNodeRef<T>) {
        if let Some(n) = node {
            Self::in_order(&n.borrow().left);
            println!("{}", n.borrow().value);
            Self::in_order(&n.borrow().right);
        }
    }
}

fn main() {
    let mut tree = BinaryTree::new();
    tree.insert(5);
    tree.insert(3);
    tree.insert(7);
    tree.insert(2);
    tree.insert(4);

    println!("In-order traversal:");
    tree.in_order_traversal();
}

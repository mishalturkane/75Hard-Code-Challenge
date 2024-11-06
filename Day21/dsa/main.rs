use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, PartialEq, Eq)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        TreeNode { val, left: None, right: None }
    }
}

// 1. Find Maximum Value in Binary Tree
fn find_max(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        Some(node) => {
            let left_max = find_max(node.borrow().left.clone());
            let right_max = find_max(node.borrow().right.clone());
            i32::max(node.borrow().val, i32::max(left_max, right_max))
        }
        None => i32::MIN,
    }
}

// 2. Find Minimum Value in Binary Tree
fn find_min(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        Some(node) => {
            let left_min = find_min(node.borrow().left.clone());
            let right_min = find_min(node.borrow().right.clone());
            i32::min(node.borrow().val, i32::min(left_min, right_min))
        }
        None => i32::MAX,
    }
}

// 3. Find Height of Binary Tree
fn find_height(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        Some(node) => {
            1 + i32::max(
                find_height(node.borrow().left.clone()),
                find_height(node.borrow().right.clone()),
            )
        }
        None => 0,
    }
}

// 4. Count Nodes in Binary Tree
fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        Some(node) => {
            1 + count_nodes(node.borrow().left.clone()) + count_nodes(node.borrow().right.clone())
        }
        None => 0,
    }
}

// 5. Sum of All Nodes in Binary Tree
fn sum_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    match root {
        Some(node) => {
            node.borrow().val
                + sum_nodes(node.borrow().left.clone())
                + sum_nodes(node.borrow().right.clone())
        }
        None => 0,
    }
}

// 6. Check if Two Binary Trees are Identical
fn are_identical(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (root1, root2) {
        (Some(node1), Some(node2)) => {
            node1.borrow().val == node2.borrow().val
                && are_identical(node1.borrow().left.clone(), node2.borrow().left.clone())
                && are_identical(node1.borrow().right.clone(), node2.borrow().right.clone())
        }
        (None, None) => true,
        _ => false,
    }
}

// 7. Invert a Binary Tree
fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        Some(node) => {
            let left = invert_tree(node.borrow().left.clone());
            let right = invert_tree(node.borrow().right.clone());
            node.borrow_mut().left = right;
            node.borrow_mut().right = left;
            Some(node)
        }
        None => None,
    }
}

// 8. Find Lowest Common Ancestor of Two Nodes
fn find_lca(root: Option<Rc<RefCell<TreeNode>>>, p: i32, q: i32) -> Option<Rc<RefCell<TreeNode>>> {
    match root {
        Some(node) => {
            if node.borrow().val == p || node.borrow().val == q {
                return Some(node);
            }
            let left = find_lca(node.borrow().left.clone(), p, q);
            let right = find_lca(node.borrow().right.clone(), p, q);
            match (left, right) {
                (Some(_), Some(_)) => Some(node),
                (Some(l), None) => Some(l),
                (None, Some(r)) => Some(r),
                _ => None,
            }
        }
        None => None,
    }
}

// 9. Print Nodes at K Distance from Root
fn nodes_at_k_distance(root: Option<Rc<RefCell<TreeNode>>>, k: i32, result: &mut Vec<i32>) {
    if let Some(node) = root {
        if k == 0 {
            result.push(node.borrow().val);
        } else {
            nodes_at_k_distance(node.borrow().left.clone(), k - 1, result);
            nodes_at_k_distance(node.borrow().right.clone(), k - 1, result);
        }
    }
}

// 10. Check if a Binary Tree is Symmetric
fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    fn is_mirror(left: Option<Rc<RefCell<TreeNode>>>, right: Option<Rc<RefCell<TreeNode>>>) -> bool {
        match (left, right) {
            (Some(l), Some(r)) => {
                l.borrow().val == r.borrow().val
                    && is_mirror(l.borrow().left.clone(), r.borrow().right.clone())
                    && is_mirror(l.borrow().right.clone(), r.borrow().left.clone())
            }
            (None, None) => true,
            _ => false,
        }
    }

    match root {
        Some(node) => is_mirror(node.borrow().left.clone(), node.borrow().right.clone()),
        None => true,
    }
}

fn main() {
    // Test binary tree
    let root = Rc::new(RefCell::new(TreeNode::new(3)));
    root.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(9))));
    root.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(20))));
    root.borrow().right.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(15))));
    root.borrow().right.as_ref().unwrap().borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(7))));

    // 1. Find Maximum Value in Binary Tree
    println!("1. Maximum Value: {}", find_max(Some(root.clone())));

    // 2. Find Minimum Value in Binary Tree
    println!("2. Minimum Value: {}", find_min(Some(root.clone())));

    // 3. Find Height of Binary Tree
    println!("3. Height: {}", find_height(Some(root.clone())));

    // 4. Count Nodes in Binary Tree
    println!("4. Total Nodes: {}", count_nodes(Some(root.clone())));

    // 5. Sum of All Nodes in Binary Tree
    println!("5. Sum of Nodes: {}", sum_nodes(Some(root.clone())));

    // 6. Check if Two Trees are Identical
    let identical_tree = Some(root.clone());
    println!("6. Are Identical: {}", are_identical(Some(root.clone()), identical_tree));

    // 7. Invert a Binary Tree
    let inverted_tree = invert_tree(Some(root.clone()));
    println!("7. Inverted Tree Root: {:?}", inverted_tree.as_ref().unwrap().borrow().val);

    // 8. Find Lowest Common Ancestor of 15 and 7
    if let Some(lca) = find_lca(Some(root.clone()), 15, 7) {
        println!("8. LCA of 15 and 7: {}", lca.borrow().val);
    }

    // 9. Nodes at K Distance from Root (k = 2)
    let mut result = Vec::new();
    nodes_at_k_distance(Some(root.clone()), 2, &mut result);
    println!("9. Nodes at distance 2: {:?}", result);

    // 10. Check if Binary Tree is Symmetric
    println!("10. Is Symmetric: {}", is_symmetric(Some(root)));
}

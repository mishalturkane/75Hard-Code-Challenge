// arraydsaquestions.rs

// Definition for a binary tree node
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Box<TreeNode>>,
    pub right: Option<Box<TreeNode>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

// 1. Maximum Path Sum in Binary Tree
fn max_path_sum(root: Option<Box<TreeNode>>) -> i32 {
    fn helper(node: &Option<Box<TreeNode>>, max_sum: &mut i32) -> i32 {
        if let Some(n) = node {
            let left_sum = helper(&n.left, max_sum).max(0);
            let right_sum = helper(&n.right, max_sum).max(0);

            *max_sum = (*max_sum).max(left_sum + right_sum + n.val);

            n.val + left_sum.max(right_sum)
        } else {
            0
        }
    }

    let mut max_sum = i32::MIN;
    helper(&root, &mut max_sum);
    max_sum
}

// 2. Serialize and Deserialize a Binary Tree
fn serialize(root: Option<Box<TreeNode>>) -> String {
    fn helper(node: &Option<Box<TreeNode>>, result: &mut Vec<String>) {
        if let Some(n) = node {
            result.push(n.val.to_string());
            helper(&n.left, result);
            helper(&n.right, result);
        } else {
            result.push("null".to_string());
        }
    }

    let mut result = Vec::new();
    helper(&root, &mut result);
    result.join(",")
}

fn deserialize(data: String) -> Option<Box<TreeNode>> {
    fn helper(data: &mut std::slice::Iter<String>) -> Option<Box<TreeNode>> {
        if let Some(val) = data.next() {
            if val == "null" {
                return None;
            }

            let mut node = TreeNode::new(val.parse().unwrap());
            node.left = helper(data);
            node.right = helper(data);

            Some(Box::new(node))
        } else {
            None
        }
    }

    let mut data_iter = data.split(',').map(|s| s.to_string()).collect::<Vec<_>>().iter();
    helper(&mut data_iter)
}

// 3. Kth Smallest Element in a BST
fn kth_smallest(root: Option<Box<TreeNode>>, k: i32) -> i32 {
    fn inorder(node: &Option<Box<TreeNode>>, result: &mut Vec<i32>) {
        if let Some(n) = node {
            inorder(&n.left, result);
            result.push(n.val);
            inorder(&n.right, result);
        }
    }

    let mut result = Vec::new();
    inorder(&root, &mut result);
    result[(k - 1) as usize]
}

// 4. Lowest Common Ancestor of a Binary Tree
fn lowest_common_ancestor(
    root: Option<Box<TreeNode>>,
    p: i32,
    q: i32,
) -> Option<Box<TreeNode>> {
    if root.is_none() {
        return None;
    }

    let root_val = root.as_ref().unwrap().val;

    if root_val == p || root_val == q {
        return root;
    }

    let left = lowest_common_ancestor(root.as_ref().unwrap().left.clone(), p, q);
    let right = lowest_common_ancestor(root.as_ref().unwrap().right.clone(), p, q);

    if left.is_some() && right.is_some() {
        return root;
    }

    if left.is_some() {
        left
    } else {
        right
    }
}

// 5. Flatten a Binary Tree to Linked List
fn flatten_tree(root: &mut Option<Box<TreeNode>>) {
    fn helper(node: &mut Option<Box<TreeNode>>) -> Option<Box<TreeNode>> {
        if node.is_none() {
            return None;
        }

        let mut current = node.take();

        let left_tail = helper(&mut current.as_mut().unwrap().left);
        let right_tail = helper(&mut current.as_mut().unwrap().right);

        if let Some(mut left) = left_tail {
            left.right = current.as_mut().unwrap().right.take();
            current.as_mut().unwrap().right = Some(left);
        }

        if right_tail.is_some() {
            right_tail
        } else {
            current
        }
    }

    helper(root);
}

fn main() {
    // Example usage
    let mut root = Some(Box::new(TreeNode::new(10)));
    root.as_mut().unwrap().left = Some(Box::new(TreeNode::new(5)));
    root.as_mut().unwrap().right = Some(Box::new(TreeNode::new(15)));

    println!("Maximum Path Sum: {}", max_path_sum(root.clone()));

    let serialized = serialize(root.clone());
    println!("Serialized Tree: {}", serialized);

    let deserialized = deserialize(serialized);
    println!("Deserialized Tree: {:?}", deserialized);

    let kth = kth_smallest(root.clone(), 2);
    println!("2nd Smallest Element: {}", kth);

    let lca = lowest_common_ancestor(root.clone(), 5, 15);
    println!("Lowest Common Ancestor: {:?}", lca);

    let mut flatten_root = root.clone();
    flatten_tree(&mut flatten_root);
    println!("Flattened Tree: {:?}", flatten_root);
}

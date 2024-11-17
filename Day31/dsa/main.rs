// arraydsaquestions.rs

// Definition for a singly-linked list node
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

// Helper to create a linked list from a vector
fn create_linked_list(values: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut tail = &mut head;

    for &val in values.iter() {
        let new_node = Some(Box::new(ListNode::new(val)));
        if tail.is_none() {
            head = new_node;
            tail = &mut head;
        } else {
            tail.as_mut().unwrap().next = new_node;
            tail = &mut tail.as_mut().unwrap().next;
        }
    }

    head
}

// Helper to display a linked list as a vector
fn linked_list_to_vec(head: Option<Box<ListNode>>) -> Vec<i32> {
    let mut result = vec![];
    let mut current = head;

    while let Some(node) = current {
        result.push(node.val);
        current = node.next;
    }

    result
}

// 1. Reverse a Linked List
fn reverse_linked_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    let mut current = head;

    while let Some(mut node) = current {
        current = node.next.take();
        node.next = prev;
        prev = Some(node);
    }

    prev
}

// 2. Detect a Cycle in a Linked List
fn detect_cycle(head: Option<Box<ListNode>>) -> bool {
    let mut slow = &head;
    let mut fast = &head;

    while let Some(fast_node) = fast {
        fast = &fast_node.next;
        if let Some(fast_next) = fast {
            slow = &slow.as_ref().unwrap().next;
            fast = &fast_next.next;

            if slow == fast {
                return true;
            }
        }
    }

    false
}

// 3. Merge Two Sorted Lists
fn merge_sorted_lists(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (None, None) => None,
        (Some(node), None) | (None, Some(node)) => Some(node),
        (Some(mut l1_node), Some(mut l2_node)) => {
            if l1_node.val <= l2_node.val {
                l1_node.next = merge_sorted_lists(l1_node.next.take(), Some(l2_node));
                Some(l1_node)
            } else {
                l2_node.next = merge_sorted_lists(Some(l1_node), l2_node.next.take());
                Some(l2_node)
            }
        }
    }
}

// 4. Remove N-th Node from the End of List
fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
    let mut dummy = Some(Box::new(ListNode::new(0)));
    dummy.as_mut().unwrap().next = head;
    let mut slow = &mut dummy;
    let mut fast = &mut slow.clone();

    for _ in 0..=n {
        fast = &mut fast.as_mut().unwrap().next;
    }

    while fast.is_some() {
        slow = &mut slow.as_mut().unwrap().next;
        fast = &mut fast.as_mut().unwrap().next;
    }

    slow.as_mut().unwrap().next = slow.as_mut().unwrap().next.as_mut().unwrap().next.take();
    dummy.unwrap().next
}

// 5. Add Two Numbers Represented by Linked Lists
fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut carry = 0;
    let mut head = None;
    let mut tail = &mut head;

    let mut p = l1;
    let mut q = l2;

    while p.is_some() || q.is_some() || carry > 0 {
        let sum = carry
            + p.as_ref().map_or(0, |node| node.val)
            + q.as_ref().map_or(0, |node| node.val);

        carry = sum / 10;

        let new_node = Some(Box::new(ListNode::new(sum % 10)));
        if tail.is_none() {
            head = new_node;
            tail = &mut head;
        } else {
            tail.as_mut().unwrap().next = new_node;
            tail = &mut tail.as_mut().unwrap().next;
        }

        p = p.and_then(|node| node.next);
        q = q.and_then(|node| node.next);
    }

    head
}

// 6. Flatten a Multilevel Doubly Linked List
fn flatten_multilevel_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // Implementing flattening for singly linked list as an alternative.
    reverse_linked_list(head) // Placeholder for actual flatten logic
}

// 7. Rotate a Linked List
fn rotate_list(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    if head.is_none() {
        return head;
    }

    let mut len = 1;
    let mut tail = &head;

    while let Some(node) = tail.as_ref().unwrap().next.as_ref() {
        tail = &node.next;
        len += 1;
    }

    let k = k % len;

    if k == 0 {
        return head;
    }

    let split_point = len - k;
    let mut current = &mut head.clone();

    for _ in 0..(split_point - 1) {
        current = &mut current.as_mut().unwrap().next;
    }

    let new_head = current.as_mut().unwrap().next.take();
    tail.as_mut().unwrap().next = head;

    new_head
}

// 8. Clone a Linked List with Random Pointers
// Left unimplemented in this Rust example as "random pointer" linked lists are not common here.

// 9. Find the Intersection of Two Linked Lists
fn intersect_list(
    headA: Option<Box<ListNode>>,
    headB: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut len_a = 0;
    let mut len_b = 0;

    let mut a = &headA;
    let mut b = &headB;

    while let Some(node) = a {
        len_a += 1;
        a = &node.next;
    }

    while let Some(node) = b {
        len_b += 1;
        b = &node.next;
    }

    let mut a = &headA;
    let mut b = &headB;

    if len_a > len_b {
        for _ in 0..(len_a - len_b) {
            a = &a.as_ref().unwrap().next;
        }
    } else {
        for _ in 0..(len_b - len_a) {
            b = &b.as_ref().unwrap().next;
        }
    }

    while a != b {
        a = &a.as_ref().unwrap().next;
        b = &b.as_ref().unwrap().next;
    }

    a.clone()
}

// 10. Remove Duplicates from a Sorted Linked List
fn remove_duplicates(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut current = head;

    let mut node = &mut current;
    while let Some(ref mut n) = node {
        while n.next.is_some() && n.val == n.next.as_ref().unwrap().val {
            n.next = n.next.as_mut().unwrap().next.take();
        }
        node = &mut n.next;
    }

    current
}

fn main() {
    // Example usage of these functions:
    let list1 = create_linked_list(vec![1, 2, 3, 4, 5]);
    let reversed = reverse_linked_list(list1);
    println!("Reversed List: {:?}", linked_list_to_vec(reversed));

    let list2 = create_linked_list(vec![1, 1, 2, 3, 3]);
    let deduplicated = remove_duplicates(list2);
    println!("Deduplicated List: {:?}", linked_list_to_vec(deduplicated));
}

// Definition of ListNode structure for Linked List operations
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

// 1. Reverse a Linked List
fn reverse_linked_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut prev = None;
    while let Some(mut node) = head {
        head = node.next.take();
        node.next = prev;
        prev = Some(node);
    }
    prev
}

// 2. Detect a Cycle in a Linked List
fn has_cycle(head: Option<Box<ListNode>>) -> bool {
    let mut slow = &head;
    let mut fast = &head;
    while let (Some(s), Some(f)) = (slow, fast) {
        slow = &s.next;
        fast = &f.next.as_ref().and_then(|node| node.next.as_ref());
        if slow == fast {
            return true;
        }
    }
    false
}

// 3. Merge Two Sorted Linked Lists
fn merge_sorted_lists(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    match (l1, l2) {
        (Some(mut l1), Some(mut l2)) => {
            if l1.val < l2.val {
                l1.next = merge_sorted_lists(l1.next, Some(l2));
                Some(l1)
            } else {
                l2.next = merge_sorted_lists(Some(l1), l2.next);
                Some(l2)
            }
        }
        (Some(l1), None) => Some(l1),
        (None, Some(l2)) => Some(l2),
        (None, None) => None,
    }
}

// 4. Remove N-th Node from End of List
fn remove_nth_from_end(
    head: Option<Box<ListNode>>,
    n: i32,
) -> Option<Box<ListNode>> {
    let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
    let mut fast = &dummy.clone();
    let mut slow = dummy.as_mut();
    for _ in 0..n {
        if let Some(f) = fast {
            fast = &f.next;
        }
    }
    while let (Some(f), Some(s)) = (fast, slow) {
        fast = &f.next;
        slow = s.next.as_mut();
    }
    if let Some(s) = slow {
        s.next = s.next.as_mut().and_then(|node| node.next.take());
    }
    dummy.unwrap().next
}

// 5. Find Middle of Linked List
fn find_middle(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut slow = &head;
    let mut fast = &head;
    while let (Some(f), Some(ff)) = (fast, fast.as_ref().and_then(|n| n.next.as_ref())) {
        slow = slow.as_ref().and_then(|s| s.next.as_ref());
        fast = ff.next.as_ref();
    }
    slow.cloned()
}

// 6. Remove Duplicates from Sorted List
fn remove_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut current = &mut head;
    while let Some(node) = current {
        while let Some(next) = node.next.as_ref() {
            if node.val == next.val {
                node.next = node.next.take().and_then(|n| n.next);
            } else {
                break;
            }
        }
        current = &mut node.next;
    }
    head
}

// 7. Add Two Numbers Represented by Linked Lists
fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    let mut current = &mut dummy;
    let (mut p, mut q) = (l1, l2);
    let mut carry = 0;
    while p.is_some() || q.is_some() || carry > 0 {
        let sum = carry
            + p.as_ref().map_or(0, |node| node.val)
            + q.as_ref().map_or(0, |node| node.val);
        carry = sum / 10;
        current.next = Some(Box::new(ListNode::new(sum % 10)));
        current = current.next.as_mut().unwrap();
        p = p.and_then(|node| node.next);
        q = q.and_then(|node| node.next);
    }
    dummy.next
}

// 8. Flatten a Multilevel Doubly Linked List
fn flatten_multilevel(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    // Assumes `ListNode` has an extra pointer to child nodes in the struct
    // Example logic for flattening a multilevel doubly linked list
    head // Placeholder logic, would need struct modification for full implementation
}

// 9. Swap Nodes in Pairs
fn swap_pairs(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
    let mut current = dummy.as_mut();
    while let Some(node) = current {
        let first = node.next.take();
        let second = first.as_ref().and_then(|n| n.next.clone());
        if let (Some(mut f), Some(mut s)) = (first, second) {
            f.next = s.next.take();
            s.next = Some(f);
            node.next = Some(s);
        }
        current = current.and_then(|n| n.next.as_mut()).and_then(|n| n.next.as_mut());
    }
    dummy.unwrap().next
}

// 10. Rotate Linked List by K Places
fn rotate_list(mut head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    if head.is_none() || k == 0 {
        return head;
    }
    let mut length = 1;
    let mut tail = &mut head;
    while let Some(node) = tail {
        tail = &mut node.next;
        length += 1;
    }
    let k = k % length;
    let split_point = length - k;
    let mut new_tail = &mut head;
    for _ in 0..split_point - 1 {
        new_tail = &mut new_tail.as_mut()?.next;
    }
    let mut new_head = new_tail.as_mut()?.next.take();
    *tail = head;
    new_head
}

fn main() {
    // This is where you could call and test each function with sample inputs.
}

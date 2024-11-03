// linkedlistdsaquestions.rs

#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

// Utility function to create a linked list from a vector
fn create_linked_list(vals: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut current = &mut head;
    for &val in vals.iter().rev() {
        let mut node = Box::new(ListNode::new(val));
        node.next = current.take();
        *current = Some(node);
    }
    head
}

fn question_1_length(head: &Option<Box<ListNode>>) -> usize {
    let mut count = 0;
    let mut current = head;
    while let Some(node) = current {
        count += 1;
        current = &node.next;
    }
    count
}

fn question_2_search(head: &Option<Box<ListNode>>, target: i32) -> Option<usize> {
    let mut current = head;
    let mut position = 0;
    while let Some(node) = current {
        if node.val == target {
            return Some(position);
        }
        current = &node.next;
        position += 1;
    }
    None
}

fn question_3_find_middle(head: &Option<Box<ListNode>>) -> Option<i32> {
    let (mut slow, mut fast) = (head, head);
    while let Some(fast_node) = fast {
        fast = &fast_node.next;
        if fast.is_some() {
            fast = &fast.as_ref().unwrap().next;
            slow = &slow.as_ref().unwrap().next;
        }
    }
    slow.as_ref().map(|node| node.val)
}

fn question_4_detect_cycle(head: &Option<Box<ListNode>>) -> bool {
    let (mut slow, mut fast) = (head, head);
    while let Some(fast_node) = fast {
        fast = &fast_node.next;
        if fast.is_some() {
            fast = &fast.as_ref().unwrap().next;
            slow = &slow.as_ref().unwrap().next;
            if slow == fast {
                return true;
            }
        }
    }
    false
}

fn question_5_reverse(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let (mut prev, mut current) = (None, head);
    while let Some(mut node) = current {
        current = node.next.take();
        node.next = prev;
        prev = Some(node);
    }
    prev
}

fn question_6_remove_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut current = &mut head;
    while let Some(node) = current {
        while let Some(next) = &node.next {
            if next.val == node.val {
                node.next = next.next.clone();
            } else {
                break;
            }
        }
        current = &mut node.next;
    }
    head
}

fn question_7_nth_from_end(head: &Option<Box<ListNode>>, n: usize) -> Option<i32> {
    let mut fast = head;
    let mut slow = head;
    for _ in 0..n {
        if let Some(fast_node) = fast {
            fast = &fast_node.next;
        } else {
            return None;
        }
    }
    while let Some(fast_node) = fast {
        fast = &fast_node.next;
        slow = &slow.as_ref().unwrap().next;
    }
    slow.as_ref().map(|node| node.val)
}
fn question_8_remove_nth_from_end(head: Option<Box<ListNode>>, n: usize) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode { val: 0, next: head });
    let mut fast = &dummy;
    let mut slow = &mut dummy;

    for _ in 0..n {
        fast = &fast.next.as_ref().unwrap();
    }

    while fast.next.is_some() {
        fast = &fast.next.as_ref().unwrap();
        slow = slow.next.as_mut().unwrap();
    }

    // Using take() to safely modify the next pointer without violating borrowing rules
    slow.next = slow.next.as_mut().and_then(|node| node.next.take());
    dummy.next
}

fn question_9_merge_sorted_lists(
    list1: Option<Box<ListNode>>,
    list2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut dummy = Box::new(ListNode::new(0));
    let mut tail = &mut dummy.next;

    let (mut l1, mut l2) = (list1, list2);
    while let (Some(n1), Some(n2)) = (l1.clone(), l2.clone()) {
        if n1.val < n2.val {
            *tail = Some(n1);
            l1 = l1.unwrap().next;
        } else {
            *tail = Some(n2);
            l2 = l2.unwrap().next;
        }
        tail = &mut tail.as_mut().unwrap().next;
    }
    *tail = if l1.is_some() { l1 } else { l2 };
    dummy.next
}

fn question_10_is_palindrome(head: &Option<Box<ListNode>>) -> bool {
    let mut values = vec![];
    let mut current = head;
    while let Some(node) = current {
        values.push(node.val);
        current = &node.next;
    }
    values == values.iter().rev().cloned().collect::<Vec<_>>()
}

fn main() {
    let head = create_linked_list(vec![1, 2, 3, 4, 5]);
    
    println!("1. Length: {:?}", question_1_length(&head));
    println!("2. Search 3: {:?}", question_2_search(&head, 3));
    println!("3. Middle: {:?}", question_3_find_middle(&head));
    println!("4. Detect Cycle: {:?}", question_4_detect_cycle(&head));
    println!("5. Reverse: {:?}", question_5_reverse(head.clone()));
    println!("6. Remove Duplicates: {:?}", question_6_remove_duplicates(create_linked_list(vec![1, 2, 2, 3])));
    println!("7. Nth from End: {:?}", question_7_nth_from_end(&head, 2));
    println!("8. Remove Nth from End: {:?}", question_8_remove_nth_from_end(head.clone(), 2));
    println!("9. Merge Sorted Lists: {:?}", question_9_merge_sorted_lists(create_linked_list(vec![1, 3, 5]), create_linked_list(vec![2, 4, 6])));
    println!("10. Is Palindrome: {:?}", question_10_is_palindrome(&create_linked_list(vec![1, 2, 3, 2, 1])));
}

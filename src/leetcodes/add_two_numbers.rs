use super::Solution;

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
impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut front = Box::new(ListNode::new(0));
        let mut current = &mut front;
        let mut p1 = &l1;
        let mut p2 = &l2;
        let mut carry = false;
        loop {
            let mut total = 0;
            match (p1, p2) {
                (Some(n1), Some(n2)) => {
                    total = n1.val + n2.val + carry as i32;
                    p1 = &n1.next;
                    p2 = &n2.next;
                }
                (Some(n1), None) => {
                    total = n1.val + carry as i32;
                    p1 = &n1.next;
                }
                (None, Some(n2)) => {
                    total = n2.val + carry as i32;
                    p2 = &n2.next;
                }
                (None, None) => {
                    break;
                }
            }
            if total >= 10 {
                total %= 10;
                carry = true;
            } else {
                carry = false;
            }
            current.next = Some(Box::new(ListNode::new(total)));
            current = current.next.as_mut().unwrap();
        }
        if carry {
            current.next = Some(Box::new(ListNode::new(1)));
        }
        front.next
    }
}

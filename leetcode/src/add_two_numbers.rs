// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode {
            next: None,
            val
        }
    }
}

pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut res = None;
    let mut curr = &mut res;
    // l1, l2, val, offset
    let mut tmp = (l1, l2, 0, 0);

    loop {
        tmp = match tmp {
            (None, None, _, 0) => break,
            (None, None, _, offset) => (None, None, offset, 0),
            (Some(list), None, _, offset)| (None, Some(list), _, offset) => (list.next, None, (list.val + offset) % 10, (list.val + offset) / 10),
            (Some(l1), Some(l2), _, offset) => (l1.next, l2.next, (l1.val + l2.val + offset) % 10, (l1.val + l2.val + offset) / 10)
        };
        *curr = Some(Box::new(ListNode::new(tmp.2)));
        curr = &mut curr.as_mut().unwrap().next;
    }
    println!("{:?}", res);
    res
}

#[cfg(test)]
mod test {

    use super::*;
    // 输入：l1 = [2,4,3], l2 = [5,6,4]
    // 输出：[7,0,8]
    // 解释：342 + 465 = 807.
    #[test]
    fn test_add_1() {
        let mut l1 = ListNode::new(2);
        let mut l1_mid = ListNode::new(4);
        l1_mid.next = Some(Box::new(ListNode::new(3)));
        l1.next = Some(Box::new(l1_mid));

        let mut l2 = ListNode::new(5);
        let mut l2_mid = ListNode::new(6);
        l2_mid.next = Some(Box::new(ListNode::new(4)));
        l2.next = Some(Box::new(l2_mid));
        
        add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2)));
    }
}
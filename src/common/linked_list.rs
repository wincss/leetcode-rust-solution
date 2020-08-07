use std::collections::VecDeque;
use std::fmt::Display;
use std::iter::FromIterator;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl Display for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}]",
            self.to_vec()
                .into_iter()
                .map(|v| format!("{}", v))
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    pub fn to_vec(&self) -> Vec<i32> {
        let mut ret = vec![self.val];
        let mut next = &self.next;
        while let Some(v) = next {
            ret.push(v.val);
            next = &v.next;
        }
        ret
    }

    pub fn from_array(list: &[i32]) -> Option<Box<Self>> {
        let mut list: VecDeque<i32> = VecDeque::from_iter(list.to_vec());
        let mut ret = Box::new(Self::new(0));
        let mut head = &mut ret;
        while let Some(v) = list.pop_front() {
            head.next.replace(Box::new(Self::new(v)));
            head = head.next.as_mut().unwrap();
        }
        ret.next
    }
}

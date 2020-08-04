#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::add_two_numbers(
                ListNode::from_array(&[2, 4, 3]),
                ListNode::from_array(&[5, 6, 4]),
            ),
            ListNode::from_array(&[7, 0, 8]),
        );
    }
    #[test]
    fn case_2() {
        assert_eq!(
            Solution::add_two_numbers(
                ListNode::from_array(&[2, 4, 6]),
                ListNode::from_array(&[5, 6, 4]),
            ),
            ListNode::from_array(&[7, 0, 1, 1]),
        );
    }
    #[test]
    fn case_3() {
        assert_eq!(
            Solution::add_two_numbers(ListNode::from_array(&[0]), ListNode::from_array(&[0]),),
            ListNode::from_array(&[0]),
        );
    }
}

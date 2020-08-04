#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn case_1() {
        assert_eq!(
            Solution::rob_iii(TreeNode::from_array(&[
                Some(3),
                Some(2),
                Some(3),
                None,
                Some(3),
                None,
                Some(1)
            ])),
            7
        );
    }

    #[test]
    fn case_2() {
        assert_eq!(
            Solution::rob_iii(TreeNode::from_array(&[
                Some(3),
                Some(4),
                Some(5),
                Some(1),
                Some(3),
                None,
                Some(1)
            ])),
            9
        );
    }

    #[test]
    fn case_3() {
        assert_eq!(
            Solution::rob_iii(TreeNode::from_array(&[
                Some(4),
                Some(1),
                None,
                Some(2),
                None,
                Some(3)
            ])),
            7
        )
    }
}

use crate::*;

use std::collections::HashSet;

fn check_answer(n: i32) -> usize {
    let result: Vec<i32> = (1..n + 1).collect();
    let mut distinct = HashSet::new();
    let trees = Solution::generate_trees(n);
    let len = trees.len();
    for mut i in trees.into_iter() {
        let tree = i.take().unwrap();
        assert_eq!(tree.borrow().inorder(), result);
        distinct.insert(tree.borrow().preorder());
    }
    assert_eq!(distinct.len(), len);
    len
}

macro_rules! create_test_case {
    ($test_name: ident, $input:expr, $output_len:expr) => {
        #[test]
        fn $test_name() {
            assert_eq!(check_answer($input), $output_len);
        }
    };
}

create_test_case!(case_0, 0, 0);
create_test_case!(case_1, 1, 1);
create_test_case!(case_2, 2, 2);
create_test_case!(case_3, 3, 5);
create_test_case!(case_4, 4, 14);
create_test_case!(case_5, 8, 1430);

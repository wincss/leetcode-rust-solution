use leetcode::*;

fn main() {
    println!("Hello World!");
    println!(
        "minimal_steps = {}",
        Solution::minimal_steps(to_string_vec(&["S#O", "M..", "M.T"]))
    );
    let tree = TreeNode::from_array(&[Some(1), None, Some(2), Some(3), Some(4)]);
    println!("to_tree = {}", tree.as_ref().unwrap().borrow());
    let mut inv_tree = Solution::invert_tree(tree);
    println!("inv_tree = {}", inv_tree.as_ref().unwrap().borrow());
    Solution::flatten(&mut inv_tree);
    println!("after flatten = {}", inv_tree.as_ref().unwrap().borrow());

    let empty_tree = TreeNode::from_array(&[]);
    println!("to_tree = {:?}", empty_tree);
    let mut inv_tree = Solution::invert_tree(empty_tree);
    println!("inv_tree = {:?}", inv_tree);
    Solution::flatten(&mut inv_tree);
    println!("after flatten = {:?}", inv_tree);

    let nums = 5;

    let trees = Solution::generate_trees(nums);
    println!(
        "{} binary search trees with {} elements:",
        trees.len(),
        nums
    );
    for t in trees.iter() {
        println!("{}", t.as_ref().unwrap().borrow());
    }
    let s = stringify!(1 + 2 * 4 - 8 * 3 + 25);
    println!("{} = {}", s, Solution::calculate(s.to_string()));
}

use leetcode::*;

fn main() {
    println!("Hello World!");
    println!(
        "minimal_steps = {}",
        Solution::minimal_steps(to_string_vec(&["S#O", "M..", "M.T"]))
    );
    let tree = to_tree(&[Some(1), None, Some(2), Some(3), Some(4)]);
    println!("to_tree = {:?}", tree);
    let mut inv_tree = Solution::invert_tree(tree);
    println!("inv_tree = {:?}", inv_tree);
    Solution::flatten(&mut inv_tree);
    println!("after flatten = {:?}", inv_tree);
}

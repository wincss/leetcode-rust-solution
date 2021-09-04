use crate::*;

impl Solution {
    pub fn smallest_k(arr: Vec<i32>, k: i32) -> Vec<i32> {
        fn partition(arr: &mut Vec<i32>, left: usize, right: usize, rank: usize) {
            if right - left <= 1 {
                return;
            }
            // println!("{} {} {}", left, right, rank);
            let pivot = arr[left];
            let mut i = left;
            let mut j = left;
            let mut k = right;
            while i < k {
                // println!("i={},j={},k={},arr={:?}", i, j, k, arr);
                if arr[i] < pivot {
                    let t = arr[j];
                    arr[j] = arr[i];
                    arr[i] = t;
                    // println!("swap arr[{}] and arr[{}]", i, j);
                    i += 1;
                    j += 1;
                } else if arr[i] > pivot {
                    k -= 1;
                    let t = arr[k];
                    arr[k] = arr[i];
                    arr[i] = t;
                // println!("swap arr[{}] and arr[{}]", i, k + 1);
                } else {
                    i += 1;
                }
            }
            assert!(
                arr[left..j].iter().max().unwrap_or(&std::i32::MIN)
                    < arr[k..right].iter().min().unwrap_or(&std::i32::MAX)
            );
            if rank < j {
                partition(arr, left, j, rank);
            } else if rank >= k {
                partition(arr, k, right, rank);
            }
        }
        let mut arr = arr;
        let n = arr.len();
        let k = k as usize;
        partition(&mut arr, 0, n, k - 1);
        arr[0..k].to_vec()
    }
}

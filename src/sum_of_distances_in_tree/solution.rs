use crate::*;

impl Solution {
    pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        fn walk(
            source: usize,
            last: Option<usize>,
            ee: &Vec<Vec<usize>>,
            weight: &mut Vec<i32>,
            size: &mut Vec<i32>,
        ) {
            for &i in ee[source].iter() {
                if last.is_some() && last.unwrap() == i {
                    continue;
                }
                walk(i, Some(source), ee, weight, size);
                weight[source] += weight[i] + size[i];
                size[source] += size[i];
            }
        }
        fn transfer(
            source: usize,
            last: Option<usize>,
            ee: &Vec<Vec<usize>>,
            weight: &mut Vec<i32>,
            size: &mut Vec<i32>,
            result: &mut Vec<i32>,
        ) {
            result[source] = weight[source];
            let source_size = size[source];
            for &i in ee[source].iter() {
                if last.is_some() && last.unwrap() == i {
                    continue;
                }

                let old_weight = weight[i];
                let old_size = size[i];
                weight[source] -= weight[i] + size[i];
                size[source] -= size[i];
                weight[i] += weight[source] + size[source];
                size[i] += size[source];

                transfer(i, Some(source), ee, weight, size, result);

                weight[source] = result[source];
                size[source] = source_size;
                weight[i] = old_weight;
                size[i] = old_size;
            }
        }
        let n = n as usize;
        let mut result = vec![0; n];
        let mut size = vec![1; n];
        let mut weight = vec![0; n];
        let mut ee = vec![vec![]; n];
        for edge in edges.into_iter() {
            ee[edge[0] as usize].push(edge[1] as usize);
            ee[edge[1] as usize].push(edge[0] as usize);
        }
        walk(0, None, &ee, &mut weight, &mut size);
        transfer(0, None, &ee, &mut weight, &mut size, &mut result);
        result
    }
}

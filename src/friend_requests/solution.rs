use crate::*;
use common::algorithms::union_find::UnionFind;

impl Solution {
    pub fn friend_requests(
        _: i32,
        restrictions: Vec<Vec<i32>>,
        requests: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let mut relationship = UnionFind::new();
        let mut result = vec![];
        for pair in requests.into_iter() {
            let x = relationship.find(&pair[0]);
            let y = relationship.find(&pair[1]);
            if restrictions.iter().any(|pair| {
                let u = relationship.find(&pair[0]);
                let v = relationship.find(&pair[1]);
                (x == u && y == v) || (x == v && y == u)
            }) {
                result.push(false);
            } else {
                result.push(true);
                relationship.union(&x, &y);
            }
        }
        result
    }
}

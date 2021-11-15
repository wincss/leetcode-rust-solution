use crate::*;

use std::collections::{HashMap, HashSet};

use common::algorithms::union_find::UnionFind;

impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        let mut groups = UnionFind::new();
        let mut mail_to_name = HashMap::new();
        for item in accounts.iter() {
            let n = item.len();
            mail_to_name.insert(&item[1], &item[0]);
            for i in 2..n {
                groups.union(&&item[1], &&item[i]);
            }
        }
        let mut mailgroup = HashMap::new();
        let mut groupname = HashMap::new();
        for item in accounts.iter() {
            let n = item.len();
            for i in 1..n {
                let group = groups.find(&&item[i]);
                mailgroup
                    .entry(group)
                    .or_insert(HashSet::new())
                    .insert(item[i].clone());
                if let Some(&name) = mail_to_name.get(&item[i]) {
                    groupname.insert(group, name);
                }
            }
        }
        let mut result = vec![];
        for (key, v) in mailgroup.iter_mut() {
            result.push(vec![groupname[key].clone()]);
            let mut emails: Vec<_> = v.drain().collect();
            emails.sort();
            result.last_mut().unwrap().append(&mut emails);
        }
        result
    }
}

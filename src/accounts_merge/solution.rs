use crate::*;

use std::collections::{HashMap, HashSet};
impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        fn find<'a, T>(
            x: &'a T,
            parent: &mut HashMap<&'a T, &'a T>,
            size: &mut HashMap<&'a T, usize>,
        ) -> &'a T
        where
            T: std::cmp::Eq + std::hash::Hash,
        {
            let f = parent.entry(x).or_insert(x);
            size.entry(x).or_insert(1);
            if **f == *x {
                x
            } else {
                let p = find(*f, parent, size);
                parent.insert(x, p);
                p
            }
        }
        fn union<'a, T>(
            x: &'a T,
            y: &'a T,
            parent: &mut HashMap<&'a T, &'a T>,
            size: &mut HashMap<&'a T, usize>,
        ) where
            T: std::cmp::Eq + std::hash::Hash,
        {
            let x = find(x, parent, size);
            let y = find(y, parent, size);
            if size[&x] > size[&y] {
                parent.insert(y, x);
                size.insert(x, size[&x] + size[&y]);
            } else {
                parent.insert(x, y);
                size.insert(y, size[&x] + size[&y]);
            }
        }
        let mut parent = HashMap::new();
        let mut size = HashMap::new();
        let mut mail_to_name = HashMap::new();
        for item in accounts.iter() {
            let n = item.len();
            mail_to_name.insert(&item[1], &item[0]);
            for i in 2..n {
                union(&item[1], &item[i], &mut parent, &mut size);
            }
        }
        let mut mailgroup = HashMap::new();
        let mut groupname = HashMap::new();
        for item in accounts.iter() {
            let n = item.len();
            for i in 1..n {
                let group = find(&item[i], &mut parent, &mut size);
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

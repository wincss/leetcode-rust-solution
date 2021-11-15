use std::collections::HashMap;

pub struct UnionFind<T> {
    parent: HashMap<T, T>,
    size: HashMap<T, usize>,
}

impl<T> UnionFind<T>
where
    T: std::cmp::Eq + std::hash::Hash + Clone,
{
    pub fn new() -> Self {
        Self {
            parent: HashMap::new(),
            size: HashMap::new(),
        }
    }
    pub fn find(&mut self, x: &T) -> T {
        find(x, &mut self.parent, &mut self.size)
    }

    pub fn union(&mut self, x: &T, y: &T) -> bool {
        union(x, y, &mut self.parent, &mut self.size)
    }
}

pub fn find<T>(x: &T, parent: &mut HashMap<T, T>, size: &mut HashMap<T, usize>) -> T
where
    T: std::cmp::Eq + std::hash::Hash + Clone,
{
    let f = parent.entry(x.clone()).or_insert(x.clone());
    size.entry(x.clone()).or_insert(1);
    if *f == *x {
        x.clone()
    } else {
        let p = find(&f.clone(), parent, size);
        parent.insert(x.clone(), p.clone());
        p
    }
}
pub fn union<T>(x: &T, y: &T, parent: &mut HashMap<T, T>, size: &mut HashMap<T, usize>) -> bool
where
    T: std::cmp::Eq + std::hash::Hash + Clone,
{
    let x = find(x, parent, size);
    let y = find(y, parent, size);
    if x == y {
        return false;
    }
    if size[&x] > size[&y] {
        parent.insert(y.clone(), x.clone());
        size.insert(x.clone(), size[&x] + size[&y]);
    } else {
        parent.insert(x.clone(), y.clone());
        size.insert(y.clone(), size[&x] + size[&y]);
    }
    true
}

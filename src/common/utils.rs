use std::fmt::Display;

pub fn to_string_vec<T: Display>(list: &[T]) -> Vec<String> {
    let mut ret = vec![];
    for i in list.iter() {
        ret.push(i.to_string());
    }
    ret
}

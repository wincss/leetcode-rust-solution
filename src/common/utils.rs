use std::fmt::Display;

pub use serde_json::json;

pub fn to_string_vec<T: Display>(list: &[T]) -> Vec<String> {
    let mut ret = vec![];
    for i in list.iter() {
        ret.push(i.to_string());
    }
    ret
}

pub fn to_board<const M: usize, const N: usize>(board: [[&str; M]; N]) -> Vec<Vec<char>> {
    board
        .iter()
        .map(|row| {
            row.iter()
                .map(|s| {
                    for c in s.chars() {
                        return c;
                    }
                    unreachable!();
                })
                .collect()
        })
        .collect()
}

#[macro_export]
macro_rules! s {
    ($s:expr) => {
        String::from($s)
    };
}

#[macro_export]
macro_rules! sv {
    () => {Vec::<String>::new()};
    ($s:expr; $n:expr) => {
        vec![String::from($s);$n]
    };
    ($($s:expr),* $(,)?) => {{
        let mut v = Vec::<String>::new();
        $(v.push(String::from($s));)*
        v
    }};
}

#[macro_export]
macro_rules! vv {
    ($([$([$($x:expr),* $(,)*]),+ $(,)*]),+ $(,)*) => {{
        vec![$(vec![$(vec![$($x,)*],)*],)*]
    }};
    ($([$($x:expr),* $(,)*]),+ $(,)*) => {{
        vec![$(vec![$($x,)*],)*]
    }};
    ($($x:expr),* $(,)*) => {{
        vec![$($x,)*]
    }};
}

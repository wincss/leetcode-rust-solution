use crate::*;

use std::collections::HashMap;
impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        let sig = (numerator < 0) ^ (denominator < 0) && numerator != 0;
        let numerator = (numerator as i64).abs();
        let denominator = (denominator as i64).abs();
        let intpart = numerator / denominator;
        let mut numerator = numerator % denominator;
        let mut decimal = vec![];
        let mut remainder = HashMap::new();
        while numerator > 0 {
            if remainder.contains_key(&numerator) {
                break;
            }
            remainder.insert(numerator, decimal.len());
            decimal.push(numerator * 10 / denominator);
            numerator = numerator * 10 % denominator;
        }
        let mut result = String::new();
        if sig {
            result.push('-');
        }
        result.push_str(&intpart.to_string());
        if !decimal.is_empty() {
            result.push('.');
        }
        let loop_left = remainder.get(&numerator);
        for (idx, d) in decimal.into_iter().enumerate() {
            if loop_left == Some(&idx) {
                result.push('(');
            }
            result.push((d + 48) as u8 as char);
        }
        if loop_left.is_some() {
            result.push(')');
        }
        result
    }
}

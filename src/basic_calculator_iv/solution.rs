use crate::*;

use std::cmp::{PartialEq, PartialOrd, Reverse};
use std::collections::HashMap;
use std::fmt::{Debug, Display, Write as FmtWrite};
use std::ops::*;

#[derive(Debug, PartialEq, Eq)]
struct Expression<T> {
    item: HashMap<Vec<String>, T>,
}

impl<T> Expression<T>
where
    T: Display,
{
    pub fn to_array(&self) -> Vec<String> {
        let mut result = vec![];
        let mut keys: Vec<&Vec<String>> = self.item.keys().collect();
        keys.sort_by_key(|v| (Reverse(v.len()), *v));
        for key in keys.into_iter() {
            let mut item = String::new();
            write!(item, "{}", self.item[key]).unwrap();
            for variable in key {
                write!(item, "*{}", variable).unwrap();
            }
            result.push(item);
        }
        result
    }

    pub fn new(key: Vec<String>, value: T) -> Self {
        let mut item = HashMap::new();
        item.insert(key, value);
        Self { item }
    }
}

impl<T> Display for Expression<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut first = true;
        for v in self.to_array().into_iter() {
            if !first {
                write!(f, " + ")?;
            }
            first = false;
            write!(f, "{}", v)?;
        }
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
enum Token<'a, T> {
    Number(T),
    Operator(&'a OperatorInfo<T>),
    Parentheses(bool),
}

type F<T> = dyn Fn(&mut Vec<T>) -> ();

struct OperatorInfo<T> {
    symbol: &'static str,
    operation: Box<F<T>>,
    precedence: u8,
}

impl<T> Debug for OperatorInfo<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "\"{}\"", self.symbol)
    }
}

impl<T> PartialEq for OperatorInfo<T> {
    fn eq(&self, other: &Self) -> bool {
        self.precedence == other.precedence
    }
}

impl<T> PartialOrd for OperatorInfo<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.precedence.partial_cmp(&other.precedence)
    }
}

struct OperatorMap<T>(HashMap<&'static str, OperatorInfo<T>>);

impl<T> Deref for OperatorMap<T> {
    type Target = HashMap<&'static str, OperatorInfo<T>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for OperatorMap<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> OperatorMap<T> {
    fn add(&mut self, info: OperatorInfo<T>) {
        self.0.insert(info.symbol, info);
    }
}

impl Solution {
    pub fn basic_calculator_iv(
        expression: String,
        evalvars: Vec<String>,
        evalints: Vec<i32>,
    ) -> Vec<String> {
        fn parse_expression<'a>(
            s: &String,
            variables: HashMap<String, i32>,
            operators: &'a OperatorMap<Expression<i32>>,
        ) -> Vec<Token<'a, Expression<i32>>> {
            let mut result = vec![];
            let mut last_number = None;
            let mut last_variable: Option<String> = None;

            for c in s.chars() {
                if c.is_ascii_digit() {
                    if let Some(v) = last_variable.take() {
                        if variables.contains_key(&v) {
                            result.push(Token::Number(Expression::new(vec![], variables[&v])));
                        } else {
                            result.push(Token::Number(Expression::new(vec![v], 1)));
                        }
                    }
                    let v = last_number.get_or_insert(0);
                    *v = *v * 10 + c as u8 as i32 - 48;
                } else if c.is_ascii_alphabetic() {
                    if let Some(v) = last_number.take() {
                        result.push(Token::Number(Expression::new(vec![], v)));
                    }
                    let v = last_variable.get_or_insert(String::new());
                    v.push(c);
                } else {
                    if let Some(v) = last_number.take() {
                        result.push(Token::Number(Expression::new(vec![], v)));
                    }
                    if let Some(v) = last_variable.take() {
                        if variables.contains_key(&v) {
                            result.push(Token::Number(Expression::new(vec![], variables[&v])));
                        } else {
                            result.push(Token::Number(Expression::new(vec![v], 1)));
                        }
                    }
                    match c {
                        '(' | '[' | '{' => {
                            result.push(Token::Parentheses(true));
                        }
                        ')' | ']' | '}' => {
                            result.push(Token::Parentheses(false));
                        }
                        ' ' => {}
                        v if operators.contains_key(&v.to_string()[..]) => {
                            result
                                .push(Token::Operator(operators.get(&v.to_string()[..]).unwrap()));
                        }
                        _ => unreachable!(),
                    }
                }
            }
            if let Some(v) = last_number.take() {
                result.push(Token::Number(Expression::new(vec![], v)));
            }
            if let Some(v) = last_variable.take() {
                if variables.contains_key(&v) {
                    result.push(Token::Number(Expression::new(vec![], variables[&v])));
                } else {
                    result.push(Token::Number(Expression::new(vec![v], 1)));
                }
            }
            result
        }
        fn generate_rpn(tokens: Vec<Token<Expression<i32>>>) -> Vec<Token<Expression<i32>>> {
            let mut result = vec![];
            let mut ops = vec![];
            for i in tokens.into_iter() {
                match i {
                    j @ Token::Number(_) => result.push(j),
                    j @ Token::Parentheses(true) => ops.push(j),
                    Token::Operator(v) => {
                        while let Some(token) = ops.last() {
                            if let Token::Operator(v0) = token {
                                if v0.precedence < v.precedence {
                                    break;
                                }
                                result.push(ops.pop().unwrap())
                            } else {
                                break;
                            }
                        }
                        ops.push(Token::Operator(v));
                    }
                    Token::Parentheses(false) => {
                        while let Some(token) = ops.pop() {
                            if let Token::Parentheses(_) = token {
                                break;
                            }
                            result.push(token);
                        }
                    }
                }
            }
            while let Some(token) = ops.pop() {
                result.push(token);
            }
            result
        }
        fn calc_rpn(tokens: Vec<Token<Expression<i32>>>) -> Expression<i32> {
            let mut stack = vec![];
            for i in tokens.into_iter() {
                match i {
                    Token::Number(v) => stack.push(v),
                    Token::Operator(v) => {
                        (v.operation)(&mut stack);
                    }
                    _ => unreachable!(),
                }
            }
            assert!(stack.len() == 1);
            stack.pop().unwrap()
        }
        let mut operators = OperatorMap(HashMap::new());
        operators.add(OperatorInfo {
            symbol: "+",
            operation: Box::new(|st: &mut Vec<Expression<i32>>| {
                if st.len() < 2 {
                    return;
                }
                let mut a = st.pop().unwrap();
                let b = st.pop().unwrap();
                for (key, value) in b.item.into_iter() {
                    *a.item.entry(key).or_insert(0) += value;
                }
                a.item.retain(|_, value| *value != 0);
                st.push(a);
            }),
            precedence: 1,
        });
        operators.add(OperatorInfo {
            symbol: "-",
            operation: Box::new(|st: &mut Vec<Expression<i32>>| {
                if st.len() < 2 {
                    return;
                }
                let b = st.pop().unwrap();
                let mut a = st.pop().unwrap();
                for (key, value) in b.item.into_iter() {
                    *a.item.entry(key).or_insert(0) -= value;
                }
                a.item.retain(|_, value| *value != 0);
                st.push(a);
            }),
            precedence: 1,
        });
        operators.add(OperatorInfo {
            symbol: "*",
            operation: Box::new(|st: &mut Vec<Expression<i32>>| {
                let a = st.pop().unwrap();
                let b = st.pop().unwrap();
                let mut item = HashMap::new();
                for (a_key, &a_value) in a.item.iter() {
                    for (b_key, &b_value) in b.item.iter() {
                        let mut key = a_key.clone();
                        key.append(&mut b_key.clone());
                        key.sort();
                        *item.entry(key).or_insert(0) += a_value * b_value;
                    }
                }
                item.retain(|_, value| *value != 0);
                st.push(Expression { item });
            }),
            precedence: 2,
        });

        let mut variables = HashMap::new();
        for (k, v) in evalvars.into_iter().zip(evalints.into_iter()) {
            variables.insert(k, v);
        }
        let expr = parse_expression(&expression, variables, &operators);
        // println!("{:?}", expr);
        let rpn = generate_rpn(expr);
        // println!("{:?}", rpn);
        let mut result = calc_rpn(rpn);
        result.item.retain(|_, value| *value != 0);
        result.to_array()
    }
}

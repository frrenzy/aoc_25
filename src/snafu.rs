#![allow(dead_code)]
use crate::helpers;
use std::{collections::HashMap, iter};

pub struct Snafu {
    i64_value: i64,
    str_value: String,
}

impl Snafu {
    pub fn from_i64(number: i64) -> Self {
        let char_map: HashMap<i64, (char, i64)> = HashMap::from([
            (0, ('0', 0)),
            (1, ('1', 0)),
            (2, ('2', 0)),
            (3, ('=', 1)),
            (4, ('-', 1)),
            (5, ('0', 1)),
        ]);

        let mut remainder = 0i64;
        let mut num = 0i64;
        let number_base5 = helpers::to_base5(number);

        let res: String = number_base5
            .chars()
            .rev()
            .map(|c| {
                num = c.to_digit(10).unwrap() as i64 + remainder;
                let (digit, rem) = *char_map.get(&num).unwrap_or(&('x', 0));
                remainder = rem;
                digit
            })
            .collect::<String>();
        let res = if remainder > 0 {
            remainder.to_string() + &res.chars().rev().collect::<String>()
        } else {
            res.chars().rev().collect::<String>()
        };

        Self {
            i64_value: number,
            str_value: res,
        }
    }

    pub fn from_string(s: &str) -> Self {
        let char_map: HashMap<char, i64> =
            HashMap::from([('=', -2), ('-', -1), ('0', 0), ('1', 1), ('2', 2)]);

        let number: i64 = s
            .chars()
            .rev()
            .scan(1, |state, c| {
                let res = char_map.get(&c).unwrap_or(&0) * *state;
                *state *= 5;

                Some(res)
            })
            .sum();

        Self {
            i64_value: number,
            str_value: s.to_string(),
        }
    }

    pub fn i64_value(&self) -> i64 {
        self.i64_value
    }

    pub fn str_value(&self) -> &str {
        &self.str_value
    }
}

impl iter::Sum for Snafu {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self::from_i64(iter.map(|v| v.i64_value).sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_conversion_d_t_s() {
        let test_case = lines("test_d_t_s.txt").expect("sosatb");
        test_case.into_iter().for_each(|l| {
            let numbers: Vec<String> = l.split_whitespace().map(|s| s.trim().to_string()).collect();
            let decimal = numbers[0].parse::<i64>().unwrap();
            let snafu = Snafu::from_i64(decimal);
            let expected = &numbers[1];
            assert_eq!(&snafu.str_value(), expected);
        });
        let test_case = lines("test_s_t_d.txt").expect("sosatb");
        test_case.into_iter().for_each(|l| {
            let numbers: Vec<String> = l.split_whitespace().map(|s| s.trim().to_string()).collect();
            let decimal = numbers[1].parse::<i64>().unwrap();
            let snafu = Snafu::from_i64(decimal);
            let expected = &numbers[0];
            assert_eq!(&snafu.str_value(), expected);
        });
    }

    #[test]
    fn test_conversion_s_t_d() {
        let test_case = lines("test_d_t_s.txt").expect("sosatb");
        test_case.into_iter().for_each(|l| {
            let numbers: Vec<String> = l.split_whitespace().map(|s| s.trim().to_string()).collect();
            let expected = numbers[0].parse::<i64>().unwrap();
            let snafu = Snafu::from_string(&numbers[1]);
            assert_eq!(snafu.i64_value(), expected);
        });
        let test_case = lines("test_s_t_d.txt").expect("sosatb");
        test_case.into_iter().for_each(|l| {
            let numbers: Vec<String> = l.split_whitespace().map(|s| s.trim().to_string()).collect();
            let expected = numbers[1].parse::<i64>().unwrap();
            let snafu = Snafu::from_string(&numbers[0]);
            assert_eq!(snafu.i64_value(), expected);
        });
    }
}

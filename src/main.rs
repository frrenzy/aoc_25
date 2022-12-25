use std::{
    collections::HashMap,
    fmt,
    fs::File,
    io::{BufRead, BufReader, Result},
    iter,
    path::Path,
};

fn lines(filename: impl AsRef<Path>) -> Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn to_base5(n: i64) -> String {
    let mut result = String::new();
    let mut m = n;
    while m > 0 {
        result += &(m % 5).to_string();
        m /= 5;
    }
    result.chars().rev().collect::<String>()
}

struct Snafu {
    i64_value: i64,
    str_value: String,
}

impl Snafu {
    fn from_i64(number: i64) -> Self {
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
        let number_base5 = to_base5(number);

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

    fn from_string(s: &str) -> Self {
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
}

impl fmt::Display for Snafu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "i64: {}, snafu: {}", self.i64_value, self.str_value)
    }
}

impl iter::Sum for Snafu {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        Self::from_i64(iter.map(|v| v.i64_value).sum())
    }
}

fn main() {
    let f = lines("data_25.txt").expect("sasai");
    let sum: Snafu = f.into_iter().map(|l| Snafu::from_string(&l)).sum::<Snafu>();
    println!("{}", sum.str_value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_base5() {
        assert_eq!(to_base5(1), 1.to_string());
        assert_eq!(to_base5(2), 2.to_string());
        assert_eq!(to_base5(3), 3.to_string());
        assert_eq!(to_base5(5), 10.to_string());
        assert_eq!(to_base5(15), 30.to_string());
        assert_eq!(to_base5(26), 101.to_string());
        assert_eq!(to_base5(2022), 31042.to_string());
    }

    #[test]
    fn test_conversion_d_t_s() {
        let test_case = lines("test_d_t_s.txt").expect("sosatb");
        test_case.into_iter().for_each(|l| {
            let numbers: Vec<String> = l.split_whitespace().map(|s| s.trim().to_string()).collect();
            let decimal = numbers[0].parse::<i64>().unwrap();
            let snafu = Snafu::from_i64(decimal);
            let expected = &numbers[1];
            assert_eq!(&snafu.str_value, expected);
        });
        let test_case = lines("test_s_t_d.txt").expect("sosatb");
        test_case.into_iter().for_each(|l| {
            let numbers: Vec<String> = l.split_whitespace().map(|s| s.trim().to_string()).collect();
            let decimal = numbers[1].parse::<i64>().unwrap();
            let snafu = Snafu::from_i64(decimal);
            let expected = &numbers[0];
            assert_eq!(&snafu.str_value, expected);
        });
    }

    #[test]
    fn test_conversion_s_t_d() {
        let test_case = lines("test_d_t_s.txt").expect("sosatb");
        test_case.into_iter().for_each(|l| {
            let numbers: Vec<String> = l.split_whitespace().map(|s| s.trim().to_string()).collect();
            let expected = numbers[0].parse::<i64>().unwrap();
            let snafu = Snafu::from_string(&numbers[1]);
            assert_eq!(snafu.i64_value, expected);
        });
        let test_case = lines("test_s_t_d.txt").expect("sosatb");
        test_case.into_iter().for_each(|l| {
            let numbers: Vec<String> = l.split_whitespace().map(|s| s.trim().to_string()).collect();
            let expected = numbers[1].parse::<i64>().unwrap();
            let snafu = Snafu::from_string(&numbers[0]);
            assert_eq!(snafu.i64_value, expected);
        });
    }
}

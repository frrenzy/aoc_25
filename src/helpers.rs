pub fn to_base5(n: i64) -> String {
    let mut result = String::new();
    let mut m = n;
    while m > 0 {
        result += &(m % 5).to_string();
        m /= 5;
    }
    result.chars().rev().collect::<String>()
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
}

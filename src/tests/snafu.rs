use crate::lines;
use crate::snafu::*;

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

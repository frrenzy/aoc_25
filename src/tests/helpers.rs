use crate::helpers::*;

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

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
}

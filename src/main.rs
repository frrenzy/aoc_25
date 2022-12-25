mod helpers;
mod snafu;
#[cfg(test)]
mod tests;

use snafu::Snafu;

use std::{
    fs::File,
    io::{BufRead, BufReader, Result},
    path::Path,
};

fn lines(filename: impl AsRef<Path>) -> Result<Vec<String>> {
    BufReader::new(File::open(filename)?).lines().collect()
}

fn main() {
    let f = lines("data_25.txt").expect("sasai");
    let sum: Snafu = f.into_iter().map(|l| Snafu::from_string(&l)).sum::<Snafu>();
    println!("{}", sum.str_value());
}

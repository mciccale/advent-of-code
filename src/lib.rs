use std::io::BufRead;

pub fn rawln() -> Vec<String> {
    std::io::stdin()
        .lock()
        .lines()
        .map(Result::unwrap)
        .collect()
}

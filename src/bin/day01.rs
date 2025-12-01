use std::io::{BufRead};

fn get_delta(line: &str) -> i32 {
    line.replace("L", "-").replace("R", "").parse::<i32>().unwrap()
}

fn easy(lines: &[String]) -> i32 {
    lines.iter().fold((50, 0), |(dial, pwd), line| {
        let delta = get_delta(&line);
        (
            (dial + delta).rem_euclid(100),
            pwd + i32::from(dial == 0),
        )
    }).1
}

fn hard(lines: &[String]) -> i32 {
    lines.iter().fold((50, 0), |(dial, pwd), line| {
        let delta = get_delta(&line);
        let new_dial = dial + delta;
        (
            new_dial.rem_euclid(100),
            pwd + i32::from(new_dial <= 0 && dial != 0) + (new_dial.signum() * new_dial / 100),
        )
    }).1
}

fn main() {
    let lines: Vec<String> = std::io::stdin().lock().lines().map(Result::unwrap).collect();
    println!("Part 1: {}", easy(&lines));
    println!("Part 2: {}", hard(&lines));
}

use advent_of_code::rawln;

fn easy(lines: &[&str]) -> i32 {
    todo!("The easy problem is still not solved!");
}

fn hard(lines: &[&str]) -> i32 {
    todo!("The easy problem is still not solved!");
}

fn main() {
    let raw: String = rawln().pop().unwrap(); // input is a single line
    let ranges: Vec<&str> = raw.split(',').collect();
    println!("Part 1: {}", easy(&ranges));
    println!("Part 2: {}", hard(&ranges));
}

use advent_of_code::rawln;

fn matrix(lines: &[&str]) -> Vec<Vec<u8>> {
  lines
    .iter()
    .map(|s| {
      s.chars().map(|c| if c == '@' { 1 } else { 0 }).collect::<Vec<u8>>()
    })
    .collect::<Vec<Vec<u8>>>()
}

fn easy(matrix: &Vec<Vec<u8>>) -> u32 {
  todo!()
}

fn hard(matrix: &Vec<Vec<u8>>) -> u32 {
  todo!()
}

fn main() {
  let raw: Vec<String> = rawln();
  let lines: Vec<&str> = raw
    .iter()
    .map(String::as_str)
    .collect();
  let matrix: Vec<Vec<u8>> = matrix(&lines);
  println!("Part 1: {}", easy(&matrix));
  println!("Part 2: {}", hard(&matrix));
}

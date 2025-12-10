use advent_of_code::rawln;

fn easy(bank: &[u8]) -> u16 {
  let l = bank.len();
  let mut sol = [l-2, l-1];
  let mut i = l-1;
  while i > 0 {
    i -= 1;
    if bank[i] >= bank[sol[0]] {
      if bank[sol[0]] >= bank[sol[1]] {
        sol[1] = sol[0];
      }
      sol[0] = i;
    }
  }
  10 * ((bank[sol[0]] - b'0') as u16) + ((bank[sol[1]] - b'0') as u16)
}

fn hard(bank: &[u8]) -> u16 {
  todo!()
}

fn main() {
  let raw: Vec<String> = rawln();
  let lines: Vec<&[u8]> = raw
    .iter()
    .map(String::as_bytes)
    .collect();
  println!("Part 1: {}", lines.iter().map(|bank| easy(&bank)).sum::<u16>());
  println!("Part 2: {}", lines.iter().map(|bank| hard(&bank)).sum::<u16>());
}

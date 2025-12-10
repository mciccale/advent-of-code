use advent_of_code::rawln;
use std::ops::Range;

fn invalid_id(n: u64) -> bool {
  let s = n.to_string();
  let l = s.len();
  l % 2 == 0 && &s[..l/2] == &s[l/2..]
}

fn easy(ranges: &[Range<u64>]) -> u64 {
  ranges
    .iter()
    .map(|r| r.clone().filter(|&n| invalid_id(n)).sum::<u64>())
    .sum()
}

fn hard(ranges: &[Range<u64>]) -> u64 {
  todo!("The easy problem is still not solved!");
}

fn main() {
  let raw: String = rawln().pop().unwrap();
  let ranges: Vec<Range<u64>> = raw
    .split(',')
    .map(|r| {
      let (start, end) = r.split_once('-').unwrap();
      Range {
        start: start.parse::<u64>().unwrap(),
        end: end.parse::<u64>().unwrap() + 1,
      }
    })
    .collect();
  println!("Part 1: {}", easy(&ranges));
  // println!("Part 2: {}", hard(&ranges));
}

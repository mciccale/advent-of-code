use std::collections::HashMap;
use advent_of_code::rawln;

type Graph<'a> = HashMap<&'a str, Vec<&'a str>>;
type Memo<'a> = HashMap<(&'a str, &'a str), u128>;

fn main() {
  let raw: Vec<String> = rawln();
  let graph: Graph = raw
    .iter()
    .fold(HashMap::new(), |mut acc, l| {
      let (k, vs) = l.split_once(": ").unwrap();
      acc.insert(k, vs.split(" ").collect());
      acc
    });
  let mut memo: Memo = HashMap::new();
  println!("Part 1: {}", easy(&graph, &mut memo));
  println!("Part 2: {}", hard(&graph, &mut memo));
}

fn easy<'a>(graph: &Graph<'a>, memo: &mut Memo<'a>) -> u128 {
  count("you", "out", graph, memo)
}

fn hard<'a>(graph: &Graph<'a>, memo: &mut Memo<'a>) -> u128 {
  let n1 = count("svr", "fft", graph, memo);
  let n2 = count("fft", "dac", graph, memo);
  let n3 = count("dac", "out", graph, memo);
  let n4 = count("svr", "dac", graph, memo);
  let n5 = count("dac", "fft", graph, memo);
  let n6 = count("fft", "out", graph, memo);
  n1*n2*n3 + n4*n5*n6
}

fn count<'a>(node: &'a str, goal: &'a str, graph: &Graph<'a>, memo: &mut Memo<'a>) -> u128 {
  if node == goal {
    1
  } else if let Some(&n) = memo.get(&(node, goal)) {
    n
  } else {
    let r = if let Some(nexts) = graph.get(node) {
      nexts.iter().fold(0, |acc, next| acc + count(next, goal, graph, memo))
    } else { 0 };
    memo.insert((node, goal), r);
    r
  }
}

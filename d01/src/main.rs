use std::collections::BTreeMap;

fn main() {
    let (l, r) = parse();
    distance(&l, &r);
    similarity(&l, &r);
}

fn parse() -> (Vec<u64>, Vec<u64>) {
    let input = std::fs::read_to_string("input").unwrap();
    let line_count = input.lines().count();
    let mut in_left: Vec<u64> = vec![0; line_count];
    let mut in_right: Vec<u64> = vec![0; line_count];
    input.split_whitespace().enumerate().for_each(|(idx, i)| {
        if let Ok(v) = i.parse::<u64>() {
            if idx % 2 == 0 {
                in_left[idx / 2] = v;
            } else {
                in_right[idx / 2] = v;
            }
        }
    });
    in_left.sort();
    in_right.sort();
    (in_left, in_right)
}

fn distance(left: &[u64], right: &[u64]) {
    let mut d: u64 = 0;
    for (&a, &b) in left.iter().zip(right.iter()) {
        d += a.abs_diff(b)
    }
    println!("{}", d);
}

fn similarity(left: &[u64], right: &[u64]) {
    let mut freq: BTreeMap<u64, u64> = BTreeMap::new();
    right.iter().for_each(|&i| {
        freq.entry(i).and_modify(|v| *v += 1).or_insert(1);
    });
    let mut s: u64 = 0;
    left.iter().for_each(|&v| {
        if let Some(cnt) = freq.get(&v) {
            s += v * cnt;
        }
    });
    println!("{}", s);
}

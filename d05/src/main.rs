use std::cmp::Ordering;
use std::collections::HashSet;

fn main() {
    let input = std::fs::read_to_string("input").unwrap();
    let mut is_parsing_rules = true;
    let mut rules = vec![HashSet::<u8>::new(); 100];
    let mut correct = Vec::<Vec<u8>>::new();
    let mut wrong = Vec::<Vec<u8>>::new();
    for l in input.lines() {
        if l.trim() == "" {
            is_parsing_rules = false;
            continue;
        }
        if is_parsing_rules {
            if let Some((v1, v2)) = parse_rule(l) {
                rules[v2].insert(v1);
            }
        } else {
            let vs: Vec<_> = l.split(",").map(|n| n.parse().unwrap()).collect();
            if vs.is_sorted_by(|a, b| cmp(&rules, a, b)) {
                correct.push(vs);
            } else {
                wrong.push(vs);
            }
        }
    }

    let res = correct
        .iter()
        .map(|l| l[l.len() / 2] as usize)
        .sum::<usize>();
    println!("{}", res);

    let mut res2 = 0;
    for ele in wrong.iter_mut() {
        ele.sort_unstable_by(|a, b| sorter(&rules, a, b));
        res2 += ele[ele.len() / 2] as usize;
    }
    println!("{}", res2);
}

fn sorter(rules: &[HashSet<u8>], a: &u8, b: &u8) -> Ordering {
    match cmp(rules, a, b) {
        true => Ordering::Less,
        _ => Ordering::Greater,
    }
}

fn cmp(rules: &[HashSet<u8>], a: &u8, b: &u8) -> bool {
    rules[*b as usize].contains(a)
}

fn parse_rule(l: &str) -> Option<(u8, usize)> {
    let mut s = l.split('|');
    let v1: u8 = parse_rule_num(s.next())?;
    let v2: usize = parse_rule_num(s.next())?;
    Some((v1, v2))
}

fn parse_rule_num<T: std::str::FromStr>(s: Option<&str>) -> Option<T> {
    match s?.parse::<T>() {
        Ok(v) => Some(v),
        _ => None,
    }
}

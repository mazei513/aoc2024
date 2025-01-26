use std::{cmp::Ordering, collections::HashSet};

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
            let mut s = l.split('|');
            let v1: u8 = s.next().unwrap().parse().unwrap();
            let v2: usize = s.next().unwrap().parse().unwrap();
            rules[v2].insert(v1);
        } else {
            let ns: Vec<(_, u8)> = l
                .split(",")
                .map(|n| n.parse().unwrap())
                .enumerate()
                .collect();

            let vs = ns.iter().map(|(_, v)| *v).collect();
            if is_ok(&rules, &ns) {
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
        ele.sort_unstable_by(|a, b| match rules[*a as usize].contains(b) {
            true => Ordering::Less,
            _ => Ordering::Greater,
        });
        res2 += ele[ele.len() / 2] as usize;
    }
    println!("{}", res2);
}

fn is_ok(rules: &[HashSet<u8>], ns: &[(usize, u8)]) -> bool {
    for (idx, n) in ns {
        if ns
            .iter()
            .skip(*idx)
            .any(|(_, v)| rules[*n as usize].contains(v))
        {
            return false;
        };
    }
    true
}

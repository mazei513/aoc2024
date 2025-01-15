fn main() {
    let mut safe_count: u16 = 0;
    let input = std::fs::read_to_string("input").unwrap();
    for l in input.lines() {
        let mut levels: Vec<i8> = vec![];
        for n_str in l.split_whitespace() {
            levels.push(n_str.parse().unwrap());
        }

        if check_levels(levels) {
            safe_count += 1;
        }
    }
    println!("{}", safe_count)
}

fn check_levels(levels: Vec<i8>) -> bool {
    for skip_i in 0..levels.len() {
        let dampened: Vec<i8> = levels
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != skip_i)
            .map(|(_, n)| *n)
            .collect();
        match dampened.len() {
            0..2 => {
                return true;
            }
            2 => {
                return check_level_diff(dampened[0] - dampened[1]);
            }
            _ => {}
        }
        let mut i = 0;
        let mut ok = true;
        while i < dampened.len() - 2 {
            let d1 = dampened[i] - dampened[i + 1];
            let d2 = dampened[i + 1] - dampened[i + 2];
            if !check_level_diff(d1) || !check_level_diff(d2) || d1.signum() != d2.signum() {
                ok = false;
                break;
            }
            i += 1;
        }
        if ok {
            return true;
        }
    }
    false
}

fn check_level_diff(d: i8) -> bool {
    d != 0 && d.abs() <= 3
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_check_levels() {
        assert!(check_levels(vec![7, 6, 4, 2, 1]));
        assert!(!check_levels(vec![1, 2, 7, 8, 9]));
        assert!(!check_levels(vec![9, 7, 6, 2, 1]));
        assert!(check_levels(vec![1, 3, 2, 4, 5]));
        assert!(check_levels(vec![8, 6, 4, 4, 1]));
        assert!(check_levels(vec![1, 3, 6, 7, 9]));
        assert!(check_levels(vec![1, 5, 6, 7, 9]));
        assert!(check_levels(vec![1, 2, 3, 4, 9]));
        assert!(check_levels(vec![1, 2, 3, 4, 9, 6]));
        assert!(!check_levels(vec![36, 40, 43, 44, 45, 46, 45, 45]));
    }
}

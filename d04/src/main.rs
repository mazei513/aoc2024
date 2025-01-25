fn main() {
    let i = std::fs::read_to_string("input").unwrap();
    println!("{}", xmas_cnt(i.as_str()));
}

fn xmas_cnt(input: &str) -> usize {
    let line_count = input.lines().count();
    let line_length = input.lines().next().unwrap().len() + 1;
    let mut tot = 0;
    let max_b = (line_count - 3) * line_length;
    let max_r = line_length - 3;
    for (idx, c) in input.char_indices() {
        let eq = match c {
            'X' => "MAS",
            'S' => "AMX",
            _ => continue,
        };
        let x_pos = idx % line_length;
        let l_ok = x_pos >= 3;
        let r_ok = x_pos < max_r;
        let b_ok = idx < max_b;

        tot += (r_ok && cmp(input, idx, eq, 1)) as usize;
        tot += (b_ok && cmp(input, idx, eq, line_length)) as usize;
        tot += (b_ok && r_ok && cmp(input, idx, eq, line_length + 1)) as usize;
        tot += (b_ok && l_ok && cmp(input, idx, eq, line_length - 1)) as usize;
    }
    tot
}

fn cmp(input: &str, skip: usize, eq: &str, step: usize) -> bool {
    let a = eq.as_bytes();
    let b = input.as_bytes();
    a[0] == b[skip + step] && a[1] == b[skip + step * 2] && a[2] == b[skip + step * 3]
}

#[cfg(test)]
mod test {
    use super::xmas_cnt;

    #[test]
    fn base() {
        assert_eq!("XMAS".get(1..4), Some("MAS"))
    }

    #[test]
    fn t1() {
        let i = "..X...
.SAMX.
.A..A.
XMAS.S
.X....";
        assert_eq!(xmas_cnt(i), 4)
    }

    #[test]
    fn t2() {
        let i = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!(xmas_cnt(i), 18)
    }
    #[test]

    fn t3() {
        let i = "......
.X....
.M....
.A....
.S....";
        assert_eq!(xmas_cnt(i), 1)
    }
}

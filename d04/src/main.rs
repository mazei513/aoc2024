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

        if r_ok {
            tot += cmp(input, idx, eq, 1);
        }
        if b_ok {
            tot += cmp(input, idx, eq, line_length);
        }
        if b_ok && r_ok {
            tot += cmp(input, idx, eq, line_length + 1);
        }
        if b_ok && l_ok {
            tot += cmp(input, idx, eq, line_length - 1);
        }
    }
    tot
}

fn cmp(input: &str, skip: usize, eq: &str, step: usize) -> usize {
    let a = eq.as_bytes();
    let b = input.as_bytes();
    ((a[2] == b[skip + step * 3]) as usize)
        * ((a[1] == b[skip + step * 2]) as usize)
        * ((a[0] == b[skip + step]) as usize)
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

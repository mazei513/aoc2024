use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input").unwrap();

    println!("{}", parse_adder(input, true))
}

#[derive(Debug)]
enum State {
    Start,
    M,
    U,
    L,
    PL,
    N1,
    C,
    N2,
    D,
    O,
    DPL,
    N,
    Qu,
    T,
    DNPL,
}

fn parse_adder(input: String, with_enabled: bool) -> usize {
    let mut total: usize = 0;
    let mut parser_state = State::M;
    let mut n1 = String::new();
    let mut n2 = String::new();
    let mut enabled = true;
    for c in input.chars() {
        match parser_state {
            State::Start => match c {
                'm' => parser_state = State::M,
                'd' => parser_state = State::D,
                _ => parser_state = State::Start,
            },
            State::M => match c {
                'u' => parser_state = State::U,
                _ => parser_state = State::Start,
            },
            State::U => match c {
                'l' => parser_state = State::L,
                _ => parser_state = State::Start,
            },
            State::L => match c {
                '(' => parser_state = State::PL,
                _ => parser_state = State::Start,
            },
            State::PL => match c {
                '0'..='9' => {
                    n1.push(c);
                    parser_state = State::N1
                }
                _ => parser_state = State::Start,
            },
            State::N1 => match c {
                '0'..='9' => n1.push(c),
                ',' => parser_state = State::C,
                _ => {
                    n1.clear();
                    parser_state = State::Start
                }
            },
            State::C => match c {
                '0'..='9' => {
                    n2.push(c);
                    parser_state = State::N2
                }
                _ => {
                    n1.clear();
                    parser_state = State::Start
                }
            },
            State::N2 => match c {
                '0'..='9' => n2.push(c),
                ')' => {
                    if !with_enabled || enabled {
                        total += n1.parse::<usize>().unwrap() * n2.parse::<usize>().unwrap();
                    }
                    n1.clear();
                    n2.clear();
                    parser_state = State::Start
                }
                _ => {
                    n1.clear();
                    n2.clear();
                    parser_state = State::Start
                }
            },
            State::D => match c {
                'o' => parser_state = State::O,
                _ => parser_state = State::Start,
            },
            State::O => match c {
                '(' => parser_state = State::DPL,
                'n' => parser_state = State::N,
                _ => parser_state = State::Start,
            },
            State::DPL => match c {
                ')' => {
                    enabled = true;
                    parser_state = State::Start
                }
                _ => parser_state = State::Start,
            },
            State::N => match c {
                '\'' => parser_state = State::Qu,
                _ => parser_state = State::Start,
            },
            State::Qu => match c {
                't' => parser_state = State::T,
                _ => parser_state = State::Start,
            },
            State::T => match c {
                '(' => parser_state = State::DNPL,
                _ => parser_state = State::Start,
            },
            State::DNPL => match c {
                ')' => {
                    enabled = false;
                    parser_state = State::Start
                }
                _ => parser_state = State::Start,
            },
        }
    }
    total
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_adder() {
        // assert_eq!(
        //     parse_adder(
        //         String::from(
        //             "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"
        //         ),
        //         false
        //     ),
        //     161
        // );
        assert_eq!(
            parse_adder(
                String::from(
                    "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"
                ),
                true
            ),
            48
        );
        // assert_eq!(parse_adder(String::from("mul ( 2 , 4 )"), false), 0);
        // assert_eq!(parse_adder(String::from("mul(4*"), false), 0);
        // assert_eq!(parse_adder(String::from("mul(6,9!"), false), 0);
        // assert_eq!(
        //     parse_adder(
        //         String::from(";({where()+'what()mul(445,324)#what()select()(+mul(430,603)"),
        //         false
        //     ),
        //     (445 * 324) + (430 * 603)
        // )
    }
}

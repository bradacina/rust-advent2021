use std::io::BufRead;

fn score_corrupt(c: char) -> u16 {
    match c {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("unexpected character {}", c),
    }
}

fn score_missing_bracket(c: char) -> u8 {
    match c {
        ')' => 1,
        ']' => 2,
        '}' => 3,
        '>' => 4,
        _ => panic!("unexpected character {} encountered", c)
    }
}

fn score_missing(l: &Vec<char>) -> u64 {
    let mut total: u64 = 0;
    for c in l {
        total *= 5;
        total += score_missing_bracket(*c) as u64;
    }

    total
}

fn read_input(filename: &str) -> Vec<String> {
    let f = std::fs::File::open(filename).expect(&("cannot open ".to_owned() + filename));

    let reader = std::io::BufReader::new(f);

    reader
        .lines()
        .map(|l| l.expect("cannot read line"))
        .collect()
}

fn part1(lines: &Vec<String>) {
    let s: u32 = lines.iter().filter_map(|l| find_corrupt_char(l)).map(|c| score_corrupt(c) as u32).sum();
    println!("{}", s);
}

fn find_corrupt_char(line: &str) -> Option<char> {
    let mut stack = Vec::new();
    for c in line.chars() {
        if c == '(' || c == '{' || c == '[' || c == '<' {
            stack.push(c);
        } else {
            let last = stack.pop().expect(&format!(
                "expecting to pop a matching character for {} but the stack was empty",
                c
            ));
            match (last, c) {
                ('(', ')') => continue,
                ('[', ']') => continue,
                ('{', '}') => continue,
                ('<', '>') => continue,
                (_, _) => return Some(c),
            }
        }
    }

    None
}

fn part2(lines: &Vec<String>) {
    let mut scores: Vec<u64> = lines.iter().filter_map(|l| fill_in_incomplete_line(l))
        .map(|l| score_missing(&l)).collect();

    scores.sort();
    let len = scores.len();

    let middle_score = scores.into_iter().skip(len/2).take(1).collect::<Vec<_>>()[0];
    println!("{:?}", middle_score);
}

fn fill_in_incomplete_line(line: &str) -> Option<Vec<char>> {
    let mut stack = Vec::new();
    for c in line.chars() {
        if c == '(' || c == '{' || c == '[' || c == '<' {
            stack.push(c);
        } else {
            let last = stack.pop().expect(&format!(
                "expecting to pop a matching character for {} but the stack was empty",
                c
            ));
            match (last, c) {
                ('(', ')') => continue,
                ('[', ']') => continue,
                ('{', '}') => continue,
                ('<', '>') => continue,
                (_, _) => return None
            }
        }
    }

    if stack.len() == 0 {
        return None
    }

    let mut result = vec![];
    for c in stack.iter().rev() {
        let closing_bracket = match c {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => panic!("unexpected character {} encountered", c)
        };

        result.push(closing_bracket);
    }

    Some(result)
}

fn main() {
    let lines = read_input("input.txt");

    // part1(&lines);
    part2(&lines);
}

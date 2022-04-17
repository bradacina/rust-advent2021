use super::types::*;

use std::io::BufRead;


pub fn parse_line(line: &str) -> Instruction {
    let items: Vec<&str> = line.split_whitespace().collect();
    if items.len() < 2 {
        panic!("instruction {} contains less than 2 tokens", line);
    }

    match items[0] {
        "add" if items.len() == 3 => {
            Instruction::Add(items[1].to_owned(), items[2].to_owned())
        }
        "mod" if items.len() == 3 => {
            Instruction::Mod(items[1].to_owned(), items[2].to_owned())
        }
        "mul" if items.len() == 3 => {
            Instruction::Mul(items[1].to_owned(), items[2].to_owned())
        }
        "div" if items.len() == 3 => {
            Instruction::Div(items[1].to_owned(), items[2].to_owned())
        }
        "mod" if items.len() == 3 => {
            Instruction::Mod(items[1].to_owned(), items[2].to_owned())
        }
        "eql" if items.len() == 3 => {
            Instruction::Equal(items[1].to_owned(), items[2].to_owned())
        }
        "inp" => Instruction::Input(items[1].to_owned()),
        _ => panic!("cannot parse {}", line),
    }
}

pub fn parse_input(filename: &str) -> Vec<Instruction> {
    let f = std::fs::File::open(filename).expect(&("Cannot open file ".to_owned() + filename));
    let reader = std::io::BufReader::new(f);

    reader
        .lines()
        .map(|line_result| line_result.unwrap())
        .map(|l| parse_line(&l))
        .collect::<Vec<Instruction>>()
}

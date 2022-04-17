use std::time::Instant;

mod alu;
mod parser;
mod types;

fn main() {
    let instructions = parser::parse_input("input.txt");

    let mut alu = alu::Alu::new(instructions);

    alu.execute();
    // let mut last_timestamp = Instant::now();
    // let mut last_input_at_timestamp = 89999999999999u64;
    // for input in ((11111111111111 as u64)..(last_input_at_timestamp)).rev() {
    //     let now = Instant::now();

    //     if now.duration_since(last_timestamp).as_secs()>0 {
    //         last_timestamp = now;
    //         println!("{} numbers processed per second", last_input_at_timestamp - input);
    //         last_input_at_timestamp = input;
    //     }

        // let string_input = input.to_string();
        // if string_input.contains("0") {
        //     continue;
        // }

        // let result = alu.send_input(&string_input);
        // if result {
        //     println!("result was {}", string_input);
        //     return;
        // }
    // }
}

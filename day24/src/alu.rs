use std::collections::HashMap;

use super::types::*;

#[derive(Debug, Clone, Copy)]
struct State {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
    min: u64,
}

#[derive(Debug)]
pub struct Alu {
    instructions: Vec<Instruction>,
    states: Vec<State>,
    num_inputs: u32
}

impl State {
    fn get_value_of_register(&self, reg: &str) -> i64 {
        match reg {
            "x" => self.x,
            "y" => self.y,
            "z" => self.z,
            "w" => self.w,
            _ => i64::from_str_radix(reg, 10).unwrap(),
        }
    }

    fn set_value_of_register(&mut self, reg: &str, val: i64) {
        match reg {
            "x" => self.x = val,
            "y" => self.y = val,
            "z" => self.z = val,
            "w" => self.w = val,
            _ => panic!("unknown register {}", reg),
        }
    }
}

impl Alu {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Alu {
            instructions,
            num_inputs: 0,
            states: vec![State {
                x: 0,
                y: 0,
                z: 0,
                w: 0,
                min: 0,
            }],
        }
    }

    pub fn execute(&mut self) {
        if self.instructions.len() == 0 {
            panic!("no instructions provided");
        }

        for instruction_index in 0..self.instructions.len() {
            let instruction = self.instructions[instruction_index].clone();
            match instruction {
                Instruction::Input(r) => self.handle_input(&r),
                Instruction::Add(a, b) => self.handle_add(&a, &b),
                Instruction::Mul(a, b) => self.handle_mul(&a, &b),
                Instruction::Div(a, b) => self.handle_div(&a, &b),
                Instruction::Mod(a, b) => self.handle_mod(&a, &b),
                Instruction::Equal(a, b) => self.handle_equal(&a, &b),
            }
        }

        let result = self.states
        .iter()
        .filter(|state| state.z == 0)
        .min_by_key(|state| state.min);

        println!("min {}", result.unwrap().min);

        let i = 1;
    }

    fn handle_equal(&mut self, a: &str, b: &str) {
        self.states.iter_mut().for_each(|state| {
            let val_a = state.get_value_of_register(a);
            let val_b = state.get_value_of_register(b);
            if val_a == val_b {
                state.set_value_of_register(a, 1);
            } else {
                state.set_value_of_register(a, 0);
            }
        })
    }

    fn handle_mod(&mut self, a: &str, b: &str) {
        self.states.iter_mut().for_each(|state| {

            let a_value = state.get_value_of_register(a);
            let b_value = state.get_value_of_register(b);
    
            if a_value < 0 {
                panic!("encountered mod instruction with first parameter containing a negative number");
            }
    
            if b_value <= 0 {
                panic!("encountered mod instruction with second parameter containing a negative number or 0");
            }
    
            let modulo = a_value % b_value;
    
            state.set_value_of_register(a, modulo);
        })
    }

    fn handle_div(&mut self, a: &str, b: &str) {
        self.states.iter_mut().for_each(|state| {

            let a_value = state.get_value_of_register(a);
            let b_value = state.get_value_of_register(b);
    
            if b_value == 0 {
                panic!("encountered division by 0");
            }
            let div = a_value / b_value;
    
            state.set_value_of_register(a, div);
        })
    }

    fn handle_mul(&mut self, a: &str, b: &str) {
        self.states.iter_mut().for_each(|state| {

            let mul = state.get_value_of_register(a) * state.get_value_of_register(b);
    
            state.set_value_of_register(a, mul);
        })
    }

    fn handle_add(&mut self, a: &str, b: &str) {
        self.states.iter_mut().for_each(|state| {

            let sum = state.get_value_of_register(a) + state.get_value_of_register(b);
    
            state.set_value_of_register(a, sum);
        })
    }

    fn handle_input(&mut self, reg: &str) {
        let threshold = self.get_threshold();
        
        self.dedup_states(threshold);
        
        let mut new_states: Vec<State> = vec![];
        (*self.states).into_iter().for_each(|state| {
            for input in 1u64..=9 {
                let mut new_state = State {..*state};
                new_state.set_value_of_register(reg, input as i64);
                new_state.min = new_state.min * 10 + input;
                new_states.push(new_state);
            }
        });

        self.states = new_states;
        
        self.num_inputs+=1;
        let threshold = self.get_threshold();
        self.dedup_states(threshold);
    }

    fn get_threshold(&self) -> (u64, u64) {
        if self.num_inputs == 0 {
            return (0,9);
        }
        (3* 10u64.pow(self.num_inputs-1),
        4 * 10u64.pow(self.num_inputs-1))
    }

    fn dedup_states(&mut self, threshold: (u64, u64)) {
        let mut hash_map: HashMap<(i64, i64, i64, i64), u64> = HashMap::new();

        let (low, high) = threshold;

        (*self.states).into_iter()
        .filter(|state|state.min <= high)
        .filter(|state|state.min >= low)
        .for_each(|state|{
            let key = (state.x, state.y, state.z, state.w);
            let entry = hash_map.entry(key).or_insert(state.min);
            *entry = (*entry).min(state.min);
        });

        self.states = hash_map.into_iter().map(|((x,y,z,w),min)| State { x,y,z,w, min}).collect();
    }
}

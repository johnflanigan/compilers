use crate::common::{ComparisonType, InfixOp, Label, Symbol};
use crate::lir::{LIRAssembly, LIRFunction, LIRInstruction, LIRProgram};
use std::collections::{HashMap, HashSet};
use std::convert::TryInto;
use std::iter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    Void,
    Str(String),
    Int(i64),
}

#[derive(Debug)]
pub struct State {
    pc: usize,
    values: HashMap<Symbol, Value>,
    all_symbols: HashSet<Symbol>,
    memory: Vec<Value>,
}

impl State {
    fn insert(&mut self, s: Symbol, v: Value) {
        assert!(self.all_symbols.contains(&s));
        self.values.insert(s, v);
    }

    fn get(&mut self, s: Symbol) -> Value {
        println!("{:?}", s);
        self.values.get(&s).expect("symbol used before set").clone()
    }

    fn allocate(&mut self, size: i64) -> usize {
        assert!(size >= 0);
        self.allocate_and_memset(Value::Void, size.try_into().unwrap())
    }

    fn allocate_and_memset(&mut self, value: Value, size: i64) -> usize {
        assert!(size >= 0);
        let allocated_loc = self.memory.len();
        self.memory.append(
            &mut iter::repeat(value)
                .take(size.try_into().unwrap())
                .collect::<Vec<_>>(),
        );
        allocated_loc
    }

    fn set_mem(&mut self, value: Value, location: usize) {
        assert!(location < self.memory.len());
        self.memory[location] = value;
    }

    fn get_mem(&mut self, location: usize) -> Value {
        assert!(location < self.memory.len());
        self.memory[location].clone()
    }
}

fn find_label(instruction_listing: &Vec<LIRAssembly>, label: Label) -> usize {
    instruction_listing
        .iter()
        .enumerate()
        .find(|&a| match a {
            (_, LIRAssembly::Label(l)) => l == &label,
            (_, _) => false,
        })
        .expect("Label couldn't be found in function")
        .0
}

pub fn eval(lir: &LIRProgram) -> Value {
    eval_fn(lir, &lir.main_function, vec![])
}

fn eval_fn(lir_prog: &LIRProgram, lir: &LIRFunction, args: Vec<Value>) -> Value {
    assert_eq!(lir.arguments.len(), args.len());
    let mut state = State {
        pc: 0,
        values: lir
            .arguments
            .iter()
            .copied()
            .zip(args.into_iter())
            .collect(),
        all_symbols: lir.get_all_symbols().into_iter().collect(),
        memory: vec![],
    };
    eval_listing(lir_prog, &lir.instruction_listing, &mut state);
    state
        .values
        .get(&lir.return_symbol)
        .unwrap_or(&Value::Void)
        .clone()
}

fn eval_listing(lir: &LIRProgram, instruction_listing: &Vec<LIRAssembly>, state: &mut State) {
    while state.pc < instruction_listing.len() {
        match instruction_listing.get(state.pc).unwrap() {
            LIRAssembly::Label(_) => state.pc += 1,
            LIRAssembly::Instruction(inst) => {
                let next_label = eval_inst(lir, inst, state);
                state.pc = match next_label {
                    Some(l) => find_label(instruction_listing, l),
                    None => state.pc + 1,
                };
            }
        }
    }
}

fn eval_inst(lir: &LIRProgram, instruction: &LIRInstruction, state: &mut State) -> Option<Label> {
    let mut next_label = None;
    match instruction {
        LIRInstruction::Nop => (),
        LIRInstruction::IntLit { assign_to, value } => {
            state.insert(*assign_to, Value::Int(*value));
        }
        LIRInstruction::StringLit { assign_to, value } => {
            state.insert(*assign_to, Value::Str(value.clone()));
        }
        LIRInstruction::StoreToMemoryAtOffset {
            location,
            offset,
            value,
        } => match (state.get(*location), state.get(*offset), state.get(*value)) {
            (Value::Int(loc), Value::Int(off), value) => {
                state.set_mem(value, (loc + off).try_into().unwrap())
            }
            _ => panic!("storing to memory with non-int location or offset"),
        },
        LIRInstruction::LoadFromMemoryAtOffset {
            assign_to,
            location,
            offset,
        } => {
            if let (Value::Int(loc), Value::Int(off)) = (state.get(*location), state.get(*offset)) {
                let v = state.get_mem((loc + off).try_into().unwrap());
                state.insert(*assign_to, v);
            }
        }
        LIRInstruction::Assign { assign_to, id } => {
            let v = state.get(*id);
            state.insert(*assign_to, v);
        }
        LIRInstruction::Negate { assign_to, value } => {
            let negated = match state.get(*value) {
                Value::Int(v) => Value::Int(-v),
                _ => panic!("Negating non int"),
            };
            state.insert(*assign_to, negated);
        }
        LIRInstruction::BinaryOp {
            assign_to,
            left,
            op,
            right,
        } => {
            let value = match (state.get(*left), state.get(*right)) {
                (Value::Int(l), Value::Int(r)) => match op {
                    InfixOp::Multiply => l * r,
                    InfixOp::Divide => l / r,
                    InfixOp::Add => l + r,
                    InfixOp::Subtract => l - r,
                    InfixOp::And => l & r,
                    InfixOp::Or => l | r,
                },
                _ => panic!("Non-Int's on left or right of InfixOp"),
            };
            state.insert(*assign_to, Value::Int(value));
        }
        LIRInstruction::Call {
            assign_to,
            function_name: Label::Allocate,
            args,
        } => {
            assert_eq!(args.len(), 1);
            let size = match state.get(args[0]) {
                Value::Int(size) => size,
                _ => panic!("Allocate called with non-int"),
            };
            let v = state.allocate(size);
            state.insert(*assign_to, Value::Int(v.try_into().unwrap()));
        }
        LIRInstruction::Call {
            assign_to,
            function_name: Label::AllocateAndMemset,
            args,
        } => {
            assert_eq!(args.len(), 2);
            let size = match state.get(args[0]) {
                Value::Int(size) => size,
                _ => panic!("Allocate called with non-int"),
            };
            let value = state.get(args[1]);
            let v = state.allocate_and_memset(value, size);
            state.insert(*assign_to, Value::Int(v.try_into().unwrap()));
        }
        LIRInstruction::Call {
            assign_to,
            function_name: Label::PrintlnInt,
            args,
        }
        | LIRInstruction::Call {
            assign_to,
            function_name: Label::PrintlnString,
            args,
        } => {
            assert_eq!(args.len(), 1);
            match state.get(args[0]) {
                Value::Int(i) => println!("{}", i),
                Value::Str(s) => println!("{}", s),
                _ => panic!("Error: Printing non string"),
            }
            state.insert(*assign_to, Value::Void);
        }
        LIRInstruction::Call {
            assign_to,
            function_name: Label::PrintInt,
            args,
        }
        | LIRInstruction::Call {
            assign_to,
            function_name: Label::PrintString,
            args,
        } => {
            assert_eq!(args.len(), 1);
            match state.get(args[0]) {
                Value::Int(i) => print!("{}", i),
                Value::Str(s) => print!("{}", s),
                _ => panic!("Error: Printing non string"),
            }
            state.insert(*assign_to, Value::Void);
        }
        LIRInstruction::Call {
            assign_to,
            function_name: Label::Main,
            args,
        } => {
            let res = eval_fn(
                lir,
                &lir.main_function,
                args.iter().map(|s| state.get(*s)).collect(),
            );
            state.insert(*assign_to, res);
        }
        LIRInstruction::Call {
            assign_to,
            function_name,
            args,
        } => {
            let res = eval_fn(
                lir,
                &lir.other_functions
                    .get(function_name)
                    .expect("function not found in function call"),
                args.iter().map(|s| state.get(*s)).collect(),
            );
            state.insert(*assign_to, res);
        }
        LIRInstruction::Jump { to } => next_label = Some(*to),
        LIRInstruction::JumpC { to, condition } => {
            let jump = match (state.get(condition.left), state.get(condition.right)) {
                (Value::Int(l), Value::Int(r)) => match condition.c {
                    ComparisonType::Equal => l == r,
                    ComparisonType::NotEqual => l != r,
                    ComparisonType::GreaterThan => l > r,
                    ComparisonType::LessThan => l < r,
                    ComparisonType::GreaterThanEqual => l >= r,
                    ComparisonType::LessThanEqual => l <= r,
                },
                _ => panic!("Non-Ints in condition of JumpC"),
            };
            if jump {
                next_label = Some(*to)
            }
        }
    }
    next_label
}

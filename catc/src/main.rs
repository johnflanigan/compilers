mod common;

#[macro_use]
mod x64;

mod lir;

mod backend;

mod x64s;

use crate::common::{Comparison, ComparisonType, Label, LabelGenerator, SymbolGenerator};

use crate::lir::{LIRAssembly, LIRFunction, LIRInstruction, LIRProgram};

//use crate::simple_lir_to_x64::compile;

use crate::backend::compile;

use std::collections::HashMap;

macro_rules! linst {
    ($data: expr) => {
        LIRAssembly::Instruction($data)
    };
}

fn main() {
    use LIRInstruction::*;

    let mut sg = SymbolGenerator::new();
    let mut lg = LabelGenerator::new();

    let i1 = sg.new_symbol();
    let i2 = sg.new_symbol();
    let s = sg.new_symbol();
    let void = sg.new_symbol();

    let skip_assignment = lg.new_label();

    let example = LIRProgram {
        main_function: LIRFunction {
            arguments: vec![],
            locals: vec![void, i1, i2, s],
            return_symbol: i1,
            instruction_listing: vec![
                linst!(IntLit {
                    assign_to: i1,
                    value: 1
                }),
                linst!(IntLit {
                    assign_to: i2,
                    value: 1
                }),
                linst!(JumpC {
                    to: skip_assignment,
                    condition: Comparison {
                        left: i1,
                        c: ComparisonType::NotEqual,
                        right: i2
                    }
                }),
                linst!(StringLit {
                    assign_to: s,
                    value: "condition false".to_string()
                }),
                linst!(Call {
                    assign_to: void,
                    function_name: Label::PrintlnString,
                    args: vec![s]
                }),
                LIRAssembly::Label(skip_assignment),
            ],
        },
        other_functions: HashMap::new(),
    };

    //let serialized_lir = serde_json::to_string_pretty(&example).unwrap();
    let compiled_example = compile(example, lg, sg);
    //let serialized_x86 = serde_json::to_string_pretty(&compiled_example).unwrap();

    //print!("serialized_lir:\n{}\n", serialized_lir);
    //print!("serialized_x86:\n{}\n", serialized_x86);
    print!("{}\n", compiled_example);
}

/*
 * This is the Lower Intermediate Representation.
 */
#![allow(dead_code)]

use serde::{Deserialize, Serialize};

use std::collections::HashMap;

use crate::common::{Comparison, InfixOp, Label, Symbol};

#[derive(Debug, Serialize, Deserialize)]
pub struct LIRProgram {
    pub main_function: LIRFunction,
    pub other_functions: HashMap<Label, LIRFunction>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LIRFunction {
    pub locals: Vec<Symbol>,
    pub arguments: Vec<Symbol>,
    pub return_symbol: Symbol,
    pub instruction_listing: Vec<LIRAssembly>,
}

impl LIRFunction {
    pub fn get_all_symbols(&self) -> Vec<Symbol> {
        self.arguments
            .clone()
            .into_iter()
            .chain(self.locals.clone().into_iter())
            .chain(vec![self.return_symbol].into_iter())
            .collect()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LIRAssembly {
    Label(Label),
    Instruction(LIRInstruction),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LIRInstruction {
    Nop,
    // Do nothing, move on to next instruction.
    IntLit {
        assign_to: Symbol,
        value: i64,
    },
    // Mutate the value stored at Symbol to be the same as "value".
    StringLit {
        assign_to: Symbol,
        value: String,
    },
    // Mutate the value stored at Symbol to be the memory location of the
    // immutable string "value".
    StoreToMemoryAtOffset {
        location: Symbol,
        offset: Symbol,
        value: Symbol,
    },
    // Store the value of symbol "value" as the memory location "location + offset".
    // This is used for array and record mutation.
    LoadFromMemoryAtOffset {
        assign_to: Symbol,
        location: Symbol,
        offset: Symbol,
    },
    // Mutate "assign_to" to be the value stored at the memory location
    // "location + offset". This is used to read a record or array.
    Assign {
        assign_to: Symbol,
        id: Symbol,
    },
    // Mutate "assign_to" to be the value in "id".
    Negate {
        assign_to: Symbol,
        value: Symbol,
    },
    // Mutate "assign_to" to be the negation of "value". Note that
    // translating this to x64 is not trivial. The semantics here require
    // that value isn't mutated.
    BinaryOp {
        assign_to: Symbol,
        left: Symbol,
        op: InfixOp,
        right: Symbol,
    },
    // Mutate "assign_to" to be the binary operation of "left op right". Note that
    // translating this to x64 is not trivial. The semantics here require that left and
    // right aren't mutated.
    Call {
        assign_to: Symbol,
        function_name: Label,
        args: Vec<Symbol>,
    },
    // Call function "function_name" with arguments "args" and store return value in
    // assign_to. Note that the semantics here require the args are not mutated.
    Jump {
        to: Label,
    },
    // Continue execution at the label "to".
    JumpC {
        to: Label,
        condition: Comparison,
    },
    // Continue execution at the label "to" only id the condition is met.
}

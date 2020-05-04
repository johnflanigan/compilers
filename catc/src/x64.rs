/*
 * This is the x64 grammar.
 */

#![allow(dead_code)]
#![allow(unused_macros)]

use crate::common::Label;

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/*
 * Note that these registers are only those which are 64-bit
 * Cat only contains 64 bit numbers (and pointers)
 */
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum X64Register {
    Rax, // Not Saved - Return
    Rbx, // Saved
    Rcx, // Not Saved - 3rd argument
    Rdx, // Not Saved - 4th argument
    Rsp, // Saved - Stack Pointer
    Rbp, // Not Saved - Base Pointer
    Rsi, // Not Saved - 2nd argument
    Rdi, // Not Saved - 1st argument
    R8,  // Not Saved - 5th argument
    R9,  // Not Saved - 6th argument
    R10, // Not Saved
    R11, // Not Saved
    R12, // Saved Across Calls
    R13, // Saved Across Calls
    R14, // Saved Across Calls
    R15, // Saved Across Calls
    Rip, // Instruction pointer
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum X64opCode {
    Add,
    Sub,
    Inc,
    Dec,
    Or,
    And,
    Movq,
    Movabsq,
    Cmp,
    Lea,

    IMulq,
    IDivq,
    Neg,
    Push,
    Pop,

    Call,
    Jmp,
    Je,
    Jne,
    Jg,
    Jge,
    Jl,
    Jle,

    Ret,
    Nop,

    Shl,
    // Feel free to add opCodes
    // if you find them useful
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum X64Value {
    LabelRef(Label),
    Absolute(i64),
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Operand {
    Immediate(X64Value),
    Register(X64Register),
    MemoryImm(X64Value),
    MemoryReg(X64Register),
    MemoryOffset(X64Value, X64Register),
    MemoryScaledIndexed(X64Value, X64Register, u8, X64Register),
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum Operands {
    Zero,
    One(Operand),
    Two(Operand, Operand),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct X64Instruction {
    pub op_code: X64opCode,
    pub args: Operands,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum X64Assembly {
    Label(Label),
    Instruction(X64Instruction),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct X64Function {
    pub instruction_listing: Vec<X64Assembly>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct X64Program {
    pub main_function: X64Function,
    pub other_functions: HashMap<Label, X64Function>,
    pub string_literals: HashMap<Label, String>,
}

/* Helpful Macros */
macro_rules! nop {
    () => {
        X64Instruction {
            op_code: X64opCode::Nop,
            args: Operands::Zero,
        }
    };
}

macro_rules! ret {
    () => {
        X64Instruction {
            op_code: X64opCode::Ret,
            args: Operands::Zero,
        }
    };
}

/* Converting to assembly text file */
use std::fmt;

impl fmt::Display for X64Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Basic main only assembly generation:
        let mut program: String = format!(".globl _main\n_main:\n{}\n", self.main_function);

        for (k, v) in self.other_functions.iter() {
            program.push_str(format!("{}:\n{}", k, v).as_str());
        }

        for (k, v) in self.string_literals.iter() {
            program.push_str(format!("{}:\t.string \"{}\"\n", k, v).as_str());
        }

        write!(f, "{}", program)
    }
}

impl fmt::Display for X64Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            X64Register::Rax => "rax",
            X64Register::Rbx => "rbx",
            X64Register::Rcx => "rcx",
            X64Register::Rdx => "rdx",
            X64Register::Rsp => "rsp",
            X64Register::Rbp => "rbp",
            X64Register::Rsi => "rsi",
            X64Register::Rdi => "rdi",
            X64Register::R8 => "r8",
            X64Register::R9 => "r9",
            X64Register::R10 => "r10",
            X64Register::R11 => "r11",
            X64Register::R12 => "r12",
            X64Register::R13 => "r13",
            X64Register::R14 => "r14",
            X64Register::R15 => "r15",
            X64Register::Rip => "rip",
        };
        write!(f, "%{}", name)
    }
}

impl fmt::Display for X64opCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            X64opCode::Add => "add",
            X64opCode::Sub => "sub",
            X64opCode::IMulq => "imulq",
            X64opCode::IDivq => "idivq",
            X64opCode::Or => "or",
            X64opCode::And => "and",
            X64opCode::Movq => "movq",
            X64opCode::Neg => "negq",
            X64opCode::Push => "pushq",
            X64opCode::Pop => "popq",
            X64opCode::Call => "call",
            X64opCode::Ret => "ret",
            X64opCode::Jmp => "jmp",
            X64opCode::Je => "je",
            X64opCode::Jne => "jne",
            X64opCode::Jg => "jg",
            X64opCode::Jge => "jge",
            X64opCode::Jl => "jl",
            X64opCode::Jle => "jle",
            X64opCode::Lea => "lea",
            X64opCode::Nop => "nop",
            X64opCode::Cmp => "cmp",
            X64opCode::Shl => "shlq",
            X64opCode::Movabsq => "movabsq",
            X64opCode::Inc => "inc",
            X64opCode::Dec => "dec",
        };
        write!(f, "{}", name)
    }
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operand::Immediate(i) => write!(f, "${}", i),
            Operand::Register(r) => write!(f, "{}", r),
            Operand::MemoryImm(v) => write!(f, "{}", v),
            Operand::MemoryReg(r) => write!(f, "({})", r),
            Operand::MemoryOffset(offset, r) => write!(f, "{}({})", offset, r),
            Operand::MemoryScaledIndexed(offset, r, scale, index) => {
                write!(f, "{}({}, {}, {})", offset, r, index, scale)
            }
        }
    }
}

impl fmt::Display for X64Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            X64Value::LabelRef(l) => write!(f, "{}", l),
            X64Value::Absolute(v) => write!(f, "{}", v),
        }
    }
}

impl fmt::Display for Operands {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operands::Zero => write!(f, ""),
            Operands::One(o) => write!(f, "{}", o),
            Operands::Two(a, b) => write!(f, "{}, {}", a, b),
        }
    }
}

impl fmt::Display for X64Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\t{}", self.op_code, self.args)
    }
}

impl fmt::Display for X64Assembly {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            X64Assembly::Label(l) => write!(f, "{}:", l),
            X64Assembly::Instruction(i) => write!(f, "\t{}", i),
        }
    }
}

impl fmt::Display for X64Function {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.instruction_listing
                .iter()
                .map(|assem| {
                    let mut assem = assem.to_string();
                    assem.push_str("\n");
                    assem
                })
                .collect::<String>()
        )
    }
}

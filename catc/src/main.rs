mod common;

#[macro_use]
mod x64;

use crate::common::{
    Comparison, ComparisonType, InfixOp, Label, LabelGenerator, Symbol, SymbolGenerator,
};

use crate::x64::{
    Operand, Operands, X64Assembly, X64Function, X64Instruction, X64Program, X64Register, X64Value,
    X64opCode,
};

use crate::common::Label::Atoi;
use crate::common::Label::Printf;
use crate::common::Label::PrintlnInt;
use crate::common::Label::Uid;
use std::collections::HashMap;
use Operand::*;
use Operands::*;
use X64Assembly::*;
use X64Register::*;
use X64Value::*;
use X64opCode::*;

fn example_1() -> X64Program {
    X64Program {
        main_function: X64Function {
            instruction_listing: vec![
                Instruction(X64Instruction {
                    op_code: Push,
                    args: One(Register(Rbp)),
                }),
                Instruction(X64Instruction {
                    op_code: Movq,
                    args: Two(Register(Rsp), Register(Rbp)),
                }),
                Instruction(X64Instruction {
                    op_code: Sub,
                    args: Two(Immediate(Absolute(64)), Register(Rsp)),
                }),
                Instruction(X64Instruction {
                    op_code: Movq,
                    args: Two(Immediate(Absolute(10)), MemoryOffset(Absolute(-56), Rbp)),
                }),
                Instruction(X64Instruction {
                    op_code: Movq,
                    args: Two(Immediate(Absolute(2)), MemoryOffset(Absolute(-8), Rbp)),
                }),
                Instruction(X64Instruction {
                    op_code: Movq,
                    args: Two(MemoryOffset(Absolute(-8), Rbp), Register(R11)),
                }),
                Instruction(X64Instruction {
                    op_code: Movq,
                    args: Two(MemoryOffset(Absolute(-56), Rbp), Register(R10)),
                }),
                Instruction(X64Instruction {
                    op_code: Cmp,
                    args: Two(Register(R10), Register(R11)),
                }),
                Instruction(X64Instruction {
                    op_code: Jg,
                    args: One(MemoryImm(LabelRef(Uid(0)))),
                }),
                Instruction(X64Instruction {
                    op_code: Movq,
                    args: Two(MemoryOffset(Absolute(-56), Rbp), Register(Rdi)),
                }),
                Instruction(X64Instruction {
                    op_code: Call,
                    args: One(MemoryImm(LabelRef(PrintlnInt))),
                }),
                Instruction(X64Instruction {
                    op_code: Movq,
                    args: Two(Register(Rax), MemoryOffset(Absolute(-48), Rbp)),
                }),
                Label(Uid(0)),
                Instruction(X64Instruction {
                    op_code: Movq,
                    args: Two(MemoryOffset(Absolute(-56), Rbp), Register(Rax)),
                }),
                Instruction(X64Instruction {
                    op_code: Add,
                    args: Two(Immediate(Absolute(64)), Register(Rsp)),
                }),
                Instruction(X64Instruction {
                    op_code: Pop,
                    args: One(Register(Rbp)),
                }),
                Instruction(X64Instruction {
                    op_code: Ret,
                    args: Zero,
                }),
            ],
        },
        other_functions: HashMap::new(),
        string_literals: HashMap::new(),
    }
}

fn example_2() -> X64Program {
    let mut example = X64Program {
        main_function: X64Function {
            instruction_listing: vec![
                // pushq %rbp
                Instruction(X64Instruction {
                    op_code: Push,
                    args: One(Register(Rbp)),
                }),
                // movq %rsp, %rbp
                Instruction(X64Instruction {
                    op_code: Movq,
                    args: Two(Register(Rsp), Register(Rbp)),
                }),
                // subq $16, %rsp
                Instruction(X64Instruction {
                    op_code: Sub,
                    args: Two(Immediate(Absolute(16)), Register(Rsp)),
                }),
                // movq 8(%rsi), %rdi
                Instruction(X64Instruction {
                    op_code: Movq,
                    args: Two(MemoryOffset(Absolute(8), Rsi), Register(Rdi)),
                }),
                // call _atoi
                Instruction(X64Instruction {
                    op_code: Call,
                    args: One(MemoryImm(LabelRef(Atoi))),
                }),
                // movq %rax, %rsi
                Instruction(X64Instruction {
                    op_code: Movq,
                    args: Two(Register(Rax), Register(Rsi)),
                }),
                // movabsq $str1, %rdi
                Instruction(X64Instruction {
                    op_code: Movabsq,
                    args: Two(Immediate(LabelRef(Uid(0))), Register(Rdi)),
                }),
                // movq $0, %rdx
                Instruction(X64Instruction {
                    op_code: Movq,
                    args: Two(Immediate(Absolute(0)), Register(Rdx)),
                }),
                // movq $0, %rcx
                Instruction(X64Instruction {
                    op_code: Movq,
                    args: Two(Immediate(Absolute(0)), Register(Rcx)),
                }),
                // loop:
                Label(Uid(1)),
                // cmp %rsi, %rcx
                Instruction(X64Instruction {
                    op_code: Cmp,
                    args: Two(Register(Rsi), Register(Rcx)),
                }),
                // je print
                Instruction(X64Instruction {
                    op_code: Je,
                    args: One(MemoryImm(LabelRef(Uid(2)))),
                }),
                // inc %rcx
                Instruction(X64Instruction {
                    op_code: Inc,
                    args: One(Register(Rcx)),
                }),
                // add %rcx, %rdx
                Instruction(X64Instruction {
                    op_code: Add,
                    args: Two(Register(Rcx), Register(Rdx)),
                }),
                // jmp loop
                Instruction(X64Instruction {
                    op_code: Jmp,
                    args: One(MemoryImm(LabelRef(Uid(1)))),
                }),
                // print:
                Label(Uid(2)),
                // callq _printf
                Instruction(X64Instruction {
                    op_code: Call,
                    args: One(MemoryImm(LabelRef(Printf))),
                }),
                // movl $0, %eax
                Instruction(X64Instruction {
                    op_code: Movq,
                    args: Two(Immediate(Absolute(0)), Register(Rax)),
                }),
                // addq $16, %rsp
                Instruction(X64Instruction {
                    op_code: Add,
                    args: Two(Immediate(Absolute(16)), Register(Rsp)),
                }),
                // popq %rbp
                Instruction(X64Instruction {
                    op_code: Pop,
                    args: One(Register(Rbp)),
                }),
                // retq
                Instruction(X64Instruction {
                    op_code: Ret,
                    args: Zero,
                }),
            ],
        },
        other_functions: HashMap::new(),
        string_literals: HashMap::new(),
    };

    example
        .string_literals
        .insert(Uid(0), String::from("Sum from 1 to %d is %d\\n"));
    example
}

fn main() {
    let example = example_2();
    print!("{}\n", example);
}

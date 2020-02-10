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

fn example_3() -> X64Program {
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
                // movq %rsp, %r15
                Instruction(X64Instruction {
                    op_code: Movq,
                    args: Two(Register(Rsp), Register(R15)),
                }),
                // subq $48, %rsp
                Instruction(X64Instruction {
                    op_code: Sub,
                    args: Two(Immediate(Absolute(48)), Register(Rsp)),
                }),
                // movq $-1, %r11
                Instruction(X64Instruction {
                    op_code: Movq,
                    args: Two(Immediate(Absolute(-1)), Register(R11)),
                }),
                // movq $5, (%r15, %r11, 8)
                Instruction(X64Instruction {
                    op_code: Movq,
                    args: Two(
                        Immediate(Absolute(5)),
                        MemoryScaledIndexed(Absolute(0), R15, 8, R11),
                    ),
                }),
                // movq $-2, %r11
                Instruction(X64Instruction {
                    op_code: Movq,
                    args: Two(Immediate(Absolute(-2)), Register(R11)),
                }),
                // movq $1, (%r15, %r11, 8)
                Instruction(X64Instruction {
                    op_code: Movq,
                    args: Two(
                        Immediate(Absolute(1)),
                        MemoryScaledIndexed(Absolute(0), R15, 8, R11),
                    ),
                }),
                // movq $-3, %r11
                Instruction(X64Instruction {
                    op_code: Movq,
                    args: Two(Immediate(Absolute(-3)), Register(R11)),
                }),
                // movq $4, (%r15, %r11, 8)
                Instruction(X64Instruction {
                    op_code: Movq,
                    args: Two(
                        Immediate(Absolute(4)),
                        MemoryScaledIndexed(Absolute(0), R15, 8, R11),
                    ),
                }),
                // movq $-4, %r11
                Instruction(X64Instruction {
                    op_code: Movq,
                    args: Two(Immediate(Absolute(-4)), Register(R11)),
                }),
                // movq $2, (%r15, %r11, 8)
                Instruction(X64Instruction {
                    op_code: Movq,
                    args: Two(
                        Immediate(Absolute(2)),
                        MemoryScaledIndexed(Absolute(0), R15, 8, R11),
                    ),
                }),
                // movq $-5, %r11
                Instruction(X64Instruction {
                    op_code: Movq,
                    args: Two(Immediate(Absolute(-5)), Register(R11)),
                }),
                // movq $8, (%r15, %r11, 8)
                Instruction(X64Instruction {
                    op_code: Movq,
                    args: Two(
                        Immediate(Absolute(8)),
                        MemoryScaledIndexed(Absolute(0), R15, 8, R11),
                    ),
                }),
                // movq $5, %r12
                Instruction(X64Instruction {
                    op_code: Movq,
                    args: Two(Immediate(Absolute(5)), Register(R12)),
                }),
                // movq $0, %r13
                Instruction(X64Instruction {
                    op_code: Movq,
                    args: Two(Immediate(Absolute(0)), Register(R13)),
                }),
                // outer:
                Label(Uid(0)),
                // movq %r12, %r8
                Instruction(X64Instruction {
                    op_code: Movq,
                    args: Two(Register(R12), Register(R8)),
                }),
                // dec %r8
                Instruction(X64Instruction {
                    op_code: Dec,
                    args: One(Register(R8)),
                }),
                // cmp %r13, %r8
                Instruction(X64Instruction {
                    op_code: Cmp,
                    args: Two(Register(R13), Register(R8)),
                }),
                // jle exit_outer
                Instruction(X64Instruction {
                    op_code: Jle,
                    args: One(MemoryImm(LabelRef(Uid(4)))),
                }),
                // movq $1, %r14
                Instruction(X64Instruction {
                    op_code: Movq,
                    args: Two(Immediate(Absolute(1)), Register(R14)),
                }),
                // inner:
                Label(Uid(1)),
                // movq %r12, %r8
                Instruction(X64Instruction {
                    op_code: Movq,
                    args: Two(Register(R12), Register(R8)),
                }),
                // subq %r13, %r8
                Instruction(X64Instruction {
                    op_code: Sub,
                    args: Two(Register(R13), Register(R8)),
                }),
                // cmp %r14, %r8
                Instruction(X64Instruction {
                    op_code: Cmp,
                    args: Two(Register(R14), Register(R8)),
                }),
                // jle exit_inner
                Instruction(X64Instruction {
                    op_code: Jle,
                    args: One(MemoryImm(LabelRef(Uid(3)))),
                }),
                // movq %r14, %r9
                Instruction(X64Instruction {
                    op_code: Movq,
                    args: Two(Register(R14), Register(R9)),
                }),
                // neg %r9
                Instruction(X64Instruction {
                    op_code: Neg,
                    args: One(Register(R9)),
                }),
                // movq %r14, %r10
                Instruction(X64Instruction {
                    op_code: Movq,
                    args: Two(Register(R14), Register(R10)),
                }),
                // neg %r10
                Instruction(X64Instruction {
                    op_code: Neg,
                    args: One(Register(R10)),
                }),
                // dec %r10
                Instruction(X64Instruction {
                    op_code: Dec,
                    args: One(Register(R10)),
                }),
                // movq (%r15, %r9, 8), %rdi
                Instruction(X64Instruction {
                    op_code: Movq,
                    args: Two(MemoryScaledIndexed(Absolute(0), R15, 8, R9), Register(Rdi)),
                }),
                // movq (%r15, %r10, 8), %rsi
                Instruction(X64Instruction {
                    op_code: Movq,
                    args: Two(MemoryScaledIndexed(Absolute(0), R15, 8, R10), Register(Rsi)),
                }),
                // cmp %rdi, %rsi
                Instruction(X64Instruction {
                    op_code: Cmp,
                    args: Two(Register(Rdi), Register(Rsi)),
                }),
                // jl swap
                Instruction(X64Instruction {
                    op_code: Jl,
                    args: One(MemoryImm(LabelRef(Uid(5)))),
                }),
                // exit_swap:
                Label(Uid(2)),
                // inc %r14
                Instruction(X64Instruction {
                    op_code: Inc,
                    args: One(Register(R14)),
                }),
                // jmp inner
                Instruction(X64Instruction {
                    op_code: Jmp,
                    args: One(MemoryImm(LabelRef(Uid(1)))),
                }),
                // exit_inner:
                Label(Uid(3)),
                // inc %r13
                Instruction(X64Instruction {
                    op_code: Inc,
                    args: One(Register(R13)),
                }),
                // jmp outer
                Instruction(X64Instruction {
                    op_code: Jmp,
                    args: One(MemoryImm(LabelRef(Uid(0)))),
                }),
                // exit_outer:
                Label(Uid(4)),
                // movabsq $str1, %rdi
                Instruction(X64Instruction {
                    op_code: Movabsq,
                    args: Two(Immediate(LabelRef(Uid(6))), Register(Rdi)),
                }),
                // movq $-1, %r11
                Instruction(X64Instruction {
                    op_code: Movq,
                    args: Two(Immediate(Absolute(-1)), Register(R11)),
                }),
                // movq (%r15, %r11, 8), %rsi
                Instruction(X64Instruction {
                    op_code: Movq,
                    args: Two(MemoryScaledIndexed(Absolute(0), R15, 8, R11), Register(Rsi)),
                }),
                // movq $-2, %r11
                Instruction(X64Instruction {
                    op_code: Movq,
                    args: Two(Immediate(Absolute(-2)), Register(R11)),
                }),
                // movq (%r15, %r11, 8), %rdx
                Instruction(X64Instruction {
                    op_code: Movq,
                    args: Two(MemoryScaledIndexed(Absolute(0), R15, 8, R11), Register(Rdx)),
                }),
                // movq $-3, %r11
                Instruction(X64Instruction {
                    op_code: Movq,
                    args: Two(Immediate(Absolute(-3)), Register(R11)),
                }),
                // movq (%r15, %r11, 8), %rcx
                Instruction(X64Instruction {
                    op_code: Movq,
                    args: Two(MemoryScaledIndexed(Absolute(0), R15, 8, R11), Register(Rcx)),
                }),
                // movq $-4, %r11
                Instruction(X64Instruction {
                    op_code: Movq,
                    args: Two(Immediate(Absolute(-4)), Register(R11)),
                }),
                // movq (%r15, %r11, 8), %r8
                Instruction(X64Instruction {
                    op_code: Movq,
                    args: Two(MemoryScaledIndexed(Absolute(0), R15, 8, R11), Register(R8)),
                }),
                // movq $-5, %r11
                Instruction(X64Instruction {
                    op_code: Movq,
                    args: Two(Immediate(Absolute(-5)), Register(R11)),
                }),
                // movq (%r15, %r11, 8), %r9
                Instruction(X64Instruction {
                    op_code: Movq,
                    args: Two(MemoryScaledIndexed(Absolute(0), R15, 8, R11), Register(R9)),
                }),
                // callq _printf
                Instruction(X64Instruction {
                    op_code: Call,
                    args: One(MemoryImm(LabelRef(Printf))),
                }),
                // addq $48, %rsp
                Instruction(X64Instruction {
                    op_code: Add,
                    args: Two(Immediate(Absolute(48)), Register(Rsp)),
                }),
                // movq $0, %rax
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
                // swap:
                Label(Uid(5)),
                // movq %rdi, (%r15, %r10, 8)
                Instruction(X64Instruction {
                    op_code: Movq,
                    args: Two(Register(Rdi), MemoryScaledIndexed(Absolute(0), R15, 8, R10)),
                }),
                // movq %rsi, (%r15, %r9, 8)
                Instruction(X64Instruction {
                    op_code: Movq,
                    args: Two(Register(Rsi), MemoryScaledIndexed(Absolute(0), R15, 8, R9)),
                }),
                // jmp exit_swap
                Instruction(X64Instruction {
                    op_code: Jmp,
                    args: One(MemoryImm(LabelRef(Uid(2)))),
                }),
            ],
        },
        other_functions: HashMap::new(),
        string_literals: HashMap::new(),
    };

    example
        .string_literals
        .insert(Uid(6), String::from("Sorted array: %d, %d, %d, %d, %d\\n"));
    example
}

fn main() {
    let example = example_3();
    print!("{}\n", example);
}

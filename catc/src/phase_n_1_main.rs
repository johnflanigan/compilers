mod common;

#[macro_use]
mod x64;

mod lir;

mod backend;

mod x64s;

use crate::common::{Comparison, ComparisonType, Label, LabelGenerator, SymbolGenerator};

use crate::lir::{LIRAssembly, LIRFunction, LIRInstruction, LIRProgram};

use crate::x64s::X64SProgram;

use crate::backend::compile;

use std::collections::HashMap;

macro_rules! linst {
    ($data: expr) => {
        LIRAssembly::Instruction($data)
    };
}

fn main() {
    use LIRInstruction::*;

    let example = "(main_function:(body:[Instruction((op_code:Movq,args:Two(Immediate(Absolute(10)),Symbol((uid:1,)),),)),Instruction((op_code:Movq,args:Two(Symbol((uid:1,)),Register(Rdi),),)),Instruction((op_code:Call,args:One(MemoryImm(LabelRef(Uid(0)))),)),Instruction((op_code:Movq,args:Two(Register(Rax),Symbol((uid:0,)),),)),Instruction((op_code:Movq,args:Two(Symbol((uid:1,)),Register(Rdi),),)),Instruction((op_code:Call,args:One(MemoryImm(LabelRef(PrintInt))),)),Instruction((op_code:Movq,args:Two(Register(Rax),Symbol((uid:2,)),),)),Instruction((op_code:Movq,args:Two(Symbol((uid:2,)),Register(Rax),),)),],),other_functions:{Uid(0):(body:[Instruction((op_code:Movq,args:Two(Register(Rdi),Symbol((uid:3,)),),)),Instruction((op_code:Movq,args:Two(Immediate(Absolute(1)),Symbol((uid:7,)),),)),Instruction((op_code:Movq,args:Two(Symbol((uid:3,)),Register(Rax),),)),Instruction((op_code:Movq,args:Two(Immediate(Absolute(0)),Register(Rdx),),)),Instruction((op_code:Sub,args:Two(Symbol((uid:7,)),Register(Rax),),)),Instruction((op_code:Movq,args:Two(Register(Rax),Symbol((uid:4,)),),)),Instruction((op_code:Movq,args:Two(Symbol((uid:3,)),Register(Rdi),),)),Instruction((op_code:Call,args:One(MemoryImm(LabelRef(Uid(0)))),)),Instruction((op_code:Movq,args:Two(Register(Rax),Symbol((uid:5,)),),)),Instruction((op_code:Movq,args:Two(Symbol((uid:4,)),Register(Rdi),),)),Instruction((op_code:Call,args:One(MemoryImm(LabelRef(Uid(0)))),)),Instruction((op_code:Movq,args:Two(Register(Rax),Symbol((uid:6,)),),)),Instruction((op_code:Movq,args:Two(Symbol((uid:6,)),Register(Rax),),)),Instruction((op_code:Movq,args:Two(Immediate(Absolute(0)),Register(Rdx),),)),Instruction((op_code:Add,args:Two(Symbol((uid:5,)),Register(Rax),),)),Instruction((op_code:Movq,args:Two(Register(Rax),Symbol((uid:7,)),),)),Instruction((op_code:Movq,args:Two(Symbol((uid:7,)),Register(Rax),),)),],),},string_literals:{},)";

    let example_program: X64SProgram = ron::de::from_str(&example).unwrap();

    // print!("Example Program:\n{:?}\n", example_program);

    // Test code for intermediate X64SProgram
    let single_memory_op = crate::backend::fix_up(example_program);
    let assigned_to_stack = crate::backend::assign_homes(single_memory_op);
    print!("{}\n", assigned_to_stack);
}

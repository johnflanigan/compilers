// Remove the flowing compiler directives and check warning prior to submitting
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use crate::common::{ComparisonType, InfixOp, Label, LabelGenerator, Symbol, SymbolGenerator};

use crate::lir::{LIRAssembly, LIRFunction, LIRInstruction, LIRProgram};

use crate::x64::{
    Operand, Operands, X64Assembly, X64Function, X64Instruction, X64Program, X64Register, X64Value,
    X64opCode,
};

use crate::x64s::{SOperand, SOperands, X64SAssembly, X64SFunction, X64SInstruction, X64SProgram};

use std::collections::{HashMap, HashSet};
use std::convert::TryInto;

#[derive(Debug)]
pub struct GlobalInfo {
    label_gen: LabelGenerator,
    symbol_gen: SymbolGenerator,
    string_literals: HashMap<Label, String>,
}

/*
 * Compile
 *
 * Convert LIRProgram to X64Program.
 *
 * Input:
 *      lir_program:
 *          the program
 *      label_gen:
 *          label generator that generates new, valid labels for the program
 *      symbol_gen:
 *          symbol generator that generates new, valid symbols for the program
 * Output:
 *      the compiled program
 */
pub fn compile(
    lir_program: LIRProgram,
    label_gen: LabelGenerator,
    symbol_gen: SymbolGenerator,
) -> X64Program {
    let mut state = GlobalInfo {
        label_gen,
        symbol_gen,
        string_literals: HashMap::new(),
    };

    let selected_program = select(lir_program, &mut state);
    let single_memory_op = fix_up(selected_program);
    let assigned_to_stack = assign_homes(single_memory_op);

    assigned_to_stack
}

/*
 * Convert LIRProgram to X64SProgram.
 *
 * Input:
 *      Any LIR program with a GlobalInfo struct that will generate new valid
 *      Symbols and Labels for the program and contains an empty HashMap for
 *      string literals.
 * Output:
 *      An LIRProgram in which each LIR instruction translation which uses
 *      Rax or Rdx must not include a X64SInstruction which includes
 *      potentially more than one memory op per (symbol) instruction.
 *
 *      Note: The above restraint exists because instructions which use
 *      more than one memory op per instruction will be fixed by fixup using
 *      Rax and Rdx. If you used these registers in the translation of a single
 *      LIR instruction then the later stages might clobber your careful use of
 *      Rax and Rdx.
 */
fn select(code: LIRProgram, state: &mut GlobalInfo) -> X64SProgram {
    unimplemented!("Homework 3");
}

/*
 * Fix Up
 *
 * Input:
 *      X64SProgram with no more than one memory indirect operand per
 *      instruction.
 * Output:
 *      X64SProgram with at most one potential memory op (symbol) per
 *      instruction.
 */
pub fn fix_up(program: X64SProgram) -> X64SProgram {
    let mut fixed_program = X64SProgram {
        main_function: fix_up_fn(&program.main_function),
        other_functions: HashMap::new(),
        string_literals: program.string_literals,
    };

    for (label, function) in program.other_functions.iter() {
        fixed_program
            .other_functions
            .insert(*label, fix_up_fn(function));
    }

    fixed_program
}

/*
 * Fix Up Function
 *
 * Input:
 *      X64SFunction with no more than one memory indirect operand per
 *      instruction.
 * Output:
 *      X64SFunction with at most one potential memory op (symbol) per
 *      instruction.
 */
fn fix_up_fn(function: &X64SFunction) -> X64SFunction {
    let mut fixed_function = X64SFunction { body: Vec::new() };

    for assembly in &function.body {
        match assembly {
            X64SAssembly::Instruction(instruction) => {
                match instruction.args {
                    // op 'x', 'y'
                    SOperands::Two(a @ SOperand::Symbol(_), b @ SOperand::Symbol(_)) => {
                        // mov a into rax
                        let instruction_1 = X64SInstruction {
                            op_code: X64opCode::Movq,
                            args: SOperands::Two(a, SOperand::Register(X64Register::Rax)),
                        };
                        fixed_function
                            .body
                            .push(X64SAssembly::Instruction(instruction_1));

                        // opcode rax, b
                        let instruction_2 = X64SInstruction {
                            op_code: instruction.op_code,
                            args: SOperands::Two(SOperand::Register(X64Register::Rax), b),
                        };
                        fixed_function
                            .body
                            .push(X64SAssembly::Instruction(instruction_2));
                    }
                    // op ('p'), %reg
                    SOperands::Two(a @ SOperand::MemorySym(_), b @ SOperand::Register(_)) => {
                        // mov a into rax
                        let instruction_1 = X64SInstruction {
                            op_code: X64opCode::Movq,
                            args: SOperands::Two(a, SOperand::Register(X64Register::Rax)),
                        };
                        fixed_function
                            .body
                            .push(X64SAssembly::Instruction(instruction_1));

                        // mov (rax) into rax
                        let instruction_2 = X64SInstruction {
                            op_code: X64opCode::Movq,
                            args: SOperands::Two(
                                SOperand::MemoryReg(X64Register::Rax),
                                SOperand::Register(X64Register::Rax),
                            ),
                        };
                        fixed_function
                            .body
                            .push(X64SAssembly::Instruction(instruction_2));

                        // opcode rax, b
                        let instruction_3 = X64SInstruction {
                            op_code: instruction.op_code,
                            args: SOperands::Two(SOperand::Register(X64Register::Rax), b),
                        };
                        fixed_function
                            .body
                            .push(X64SAssembly::Instruction(instruction_3));
                    }
                    // op ('p'), 'y'
                    SOperands::Two(a @ SOperand::MemorySym(_), b @ SOperand::Symbol(_)) => {
                        // mov a into rax
                        let instruction_1 = X64SInstruction {
                            op_code: X64opCode::Movq,
                            args: SOperands::Two(a, SOperand::Register(X64Register::Rax)),
                        };
                        fixed_function
                            .body
                            .push(X64SAssembly::Instruction(instruction_1));

                        // mov (rax) into rax
                        let instruction_2 = X64SInstruction {
                            op_code: X64opCode::Movq,
                            args: SOperands::Two(
                                SOperand::MemoryReg(X64Register::Rax),
                                SOperand::Register(X64Register::Rax),
                            ),
                        };
                        fixed_function
                            .body
                            .push(X64SAssembly::Instruction(instruction_2));

                        // opcode rax, b
                        let instruction_3 = X64SInstruction {
                            op_code: instruction.op_code,
                            args: SOperands::Two(SOperand::Register(X64Register::Rax), b),
                        };
                        fixed_function
                            .body
                            .push(X64SAssembly::Instruction(instruction_3));
                    }
                    // op ('p'), ('q')
                    SOperands::Two(a @ SOperand::MemorySym(_), b @ SOperand::MemorySym(_)) => {
                        // mov a into rax
                        let instruction_1 = X64SInstruction {
                            op_code: X64opCode::Movq,
                            args: SOperands::Two(a, SOperand::Register(X64Register::Rax)),
                        };
                        fixed_function
                            .body
                            .push(X64SAssembly::Instruction(instruction_1));

                        // mov (rax) into rax
                        let instruction_2 = X64SInstruction {
                            op_code: X64opCode::Movq,
                            args: SOperands::Two(
                                SOperand::MemoryReg(X64Register::Rax),
                                SOperand::Register(X64Register::Rax),
                            ),
                        };
                        fixed_function
                            .body
                            .push(X64SAssembly::Instruction(instruction_2));

                        // mov b into rdx
                        let instruction_3 = X64SInstruction {
                            op_code: X64opCode::Movq,
                            args: SOperands::Two(b, SOperand::Register(X64Register::Rdx)),
                        };
                        fixed_function
                            .body
                            .push(X64SAssembly::Instruction(instruction_3));

                        // opcode rax, (rdx)
                        let instruction_4 = X64SInstruction {
                            op_code: instruction.op_code,
                            args: SOperands::Two(
                                SOperand::Register(X64Register::Rax),
                                SOperand::MemoryReg(X64Register::Rdx),
                            ),
                        };
                        fixed_function
                            .body
                            .push(X64SAssembly::Instruction(instruction_4));
                    }
                    _ => (fixed_function.body.push(assembly.clone())),
                };
            }
            _ => (fixed_function.body.push(assembly.clone())),
        };
    }

    fixed_function
}

/*
 * Assign Homes
 *
 * For each X64SFunction, compute the register assignment and use it to
 * to assign_homes_fn.
 *
 * Input:
 *      X64SProgram with at most one potential memory op (symbol) per
 *      instruction.
 * Output:
 *      X64Program ready to run.
 */
pub fn assign_homes(program: X64SProgram) -> X64Program {
    let main_allocation = register_alloc(&program.main_function);
    let main_assignment = register_assignment(main_allocation);

    let mut compiled_program = X64Program {
        main_function: assign_homes_fn(program.main_function, main_assignment),
        other_functions: HashMap::new(),
        string_literals: program.string_literals,
    };

    for (label, function) in program.other_functions.iter() {
        let function_allocation = register_alloc(function);
        let function_assignment = register_assignment(function_allocation);
        let compiled_function = assign_homes_fn(function.clone(), function_assignment);

        compiled_program
            .other_functions
            .insert(*label, compiled_function);
    }

    compiled_program
}

/*
 * Assign Homes Function
 *
 * Input:
 *      X64SFunction with at most one potential memory op (symbol) per
 *      instruction.
 * Output:
 *      X64Function with prologue and epilogue, symbols replaced with
 *      stack offsets.
 */
fn assign_homes_fn(function: X64SFunction, homes: HashMap<Symbol, StackOrReg>) -> X64Function {
    let mut compiled_function = X64Function {
        instruction_listing: Vec::new(),
    };

    // Prologue
    compiled_function
        .instruction_listing
        .push(X64Assembly::Instruction(X64Instruction {
            op_code: X64opCode::Push,
            args: Operands::One(Operand::Register(X64Register::Rbp)),
        }));
    compiled_function
        .instruction_listing
        .push(X64Assembly::Instruction(X64Instruction {
            op_code: X64opCode::Movq,
            args: Operands::Two(
                Operand::Register(X64Register::Rsp),
                Operand::Register(X64Register::Rbp),
            ),
        }));
    let stack_reservation: i64 = ((homes.len() + 1) * 8).try_into().unwrap();
    compiled_function
        .instruction_listing
        .push(X64Assembly::Instruction(X64Instruction {
            op_code: X64opCode::Sub,
            args: Operands::Two(
                Operand::Immediate(X64Value::Absolute(stack_reservation)),
                Operand::Register(X64Register::Rsp),
            ),
        }));

    for assembly in function.body {
        match assembly {
            X64SAssembly::Label(label) => {
                compiled_function
                    .instruction_listing
                    .push(X64Assembly::Label(label));
            }
            X64SAssembly::Instruction(instruction) => {
                match instruction.args {
                    SOperands::One(SOperand::Symbol(symbol)) => {
                        let stack_or_reg = homes.get(&symbol).unwrap();
                        let operand = match stack_or_reg {
                            StackOrReg::Stack(offset) => {
                                Operand::MemoryOffset(X64Value::Absolute(*offset), X64Register::Rbp)
                            }
                            StackOrReg::Reg(register) => Operand::Register(*register),
                        };
                        let compiled_instruction = X64Assembly::Instruction(X64Instruction {
                            op_code: instruction.op_code,
                            args: Operands::One(operand),
                        });
                        compiled_function
                            .instruction_listing
                            .push(compiled_instruction);
                    }
                    SOperands::One(SOperand::MemorySym(symbol)) => {
                        let stack_or_reg = homes.get(&symbol).unwrap();
                        let operand = match stack_or_reg {
                            StackOrReg::Stack(offset) => {
                                panic!("Memory stack access are not allowed")
                            }
                            StackOrReg::Reg(register) => Operand::MemoryReg(*register),
                        };
                        let compiled_instruction = X64Assembly::Instruction(X64Instruction {
                            op_code: instruction.op_code,
                            args: Operands::One(operand),
                        });
                        compiled_function
                            .instruction_listing
                            .push(compiled_instruction);
                    }
                    SOperands::Two(SOperand::Symbol(symbol), b) => {
                        let stack_or_reg = homes.get(&symbol).unwrap();
                        let operand = match stack_or_reg {
                            StackOrReg::Stack(offset) => {
                                Operand::MemoryOffset(X64Value::Absolute(*offset), X64Register::Rbp)
                            }
                            StackOrReg::Reg(register) => Operand::Register(*register),
                        };
                        let compiled_instruction = X64Assembly::Instruction(X64Instruction {
                            op_code: instruction.op_code,
                            args: Operands::Two(operand, b.try_into().unwrap()),
                        });
                        compiled_function
                            .instruction_listing
                            .push(compiled_instruction);
                    }
                    SOperands::Two(SOperand::MemorySym(symbol), b) => {
                        let stack_or_reg = homes.get(&symbol).unwrap();
                        let operand = match stack_or_reg {
                            StackOrReg::Stack(offset) => {
                                panic!("Memory stack access are not allowed")
                            }
                            StackOrReg::Reg(register) => Operand::MemoryReg(*register),
                        };
                        let compiled_instruction = X64Assembly::Instruction(X64Instruction {
                            op_code: instruction.op_code,
                            args: Operands::Two(operand, b.try_into().unwrap()),
                        });
                        compiled_function
                            .instruction_listing
                            .push(compiled_instruction);
                    }
                    SOperands::Two(a, SOperand::Symbol(symbol)) => {
                        let stack_or_reg = homes.get(&symbol).unwrap();
                        let operand = match stack_or_reg {
                            StackOrReg::Stack(offset) => {
                                Operand::MemoryOffset(X64Value::Absolute(*offset), X64Register::Rbp)
                            }
                            StackOrReg::Reg(register) => Operand::Register(*register),
                        };
                        let compiled_instruction = X64Assembly::Instruction(X64Instruction {
                            op_code: instruction.op_code,
                            args: Operands::Two(a.try_into().unwrap(), operand),
                        });
                        compiled_function
                            .instruction_listing
                            .push(compiled_instruction);
                    }
                    SOperands::Two(a, SOperand::MemorySym(symbol)) => {
                        let stack_or_reg = homes.get(&symbol).unwrap();
                        let operand = match stack_or_reg {
                            StackOrReg::Stack(offset) => {
                                panic!("Memory stack access are not allowed")
                            }
                            StackOrReg::Reg(register) => Operand::MemoryReg(*register),
                        };
                        let compiled_instruction = X64Assembly::Instruction(X64Instruction {
                            op_code: instruction.op_code,
                            args: Operands::Two(a.try_into().unwrap(), operand),
                        });
                        compiled_function
                            .instruction_listing
                            .push(compiled_instruction);
                    }
                    SOperands::One(a) => {
                        let compiled_instruction = X64Assembly::Instruction(X64Instruction {
                            op_code: instruction.op_code,
                            args: Operands::One(a.try_into().unwrap()),
                        });
                        compiled_function
                            .instruction_listing
                            .push(compiled_instruction);
                    }
                    SOperands::Two(a, b) => {
                        let compiled_instruction = X64Assembly::Instruction(X64Instruction {
                            op_code: instruction.op_code,
                            args: Operands::Two(a.try_into().unwrap(), b.try_into().unwrap()),
                        });
                        compiled_function
                            .instruction_listing
                            .push(compiled_instruction);
                    }
                    SOperands::Zero => {
                        let compiled_instruction = X64Assembly::Instruction(X64Instruction {
                            op_code: instruction.op_code,
                            args: Operands::Zero,
                        });
                        compiled_function
                            .instruction_listing
                            .push(compiled_instruction);
                    }
                };
            }
        };
    }

    // Epilogue
    compiled_function
        .instruction_listing
        .push(X64Assembly::Instruction(X64Instruction {
            op_code: X64opCode::Movq,
            args: Operands::Two(
                Operand::Register(X64Register::Rbp),
                Operand::Register(X64Register::Rsp),
            ),
        }));
    compiled_function
        .instruction_listing
        .push(X64Assembly::Instruction(X64Instruction {
            op_code: X64opCode::Pop,
            args: Operands::One(Operand::Register(X64Register::Rbp)),
        }));
    compiled_function
        .instruction_listing
        .push(X64Assembly::Instruction(X64Instruction {
            op_code: X64opCode::Ret,
            args: Operands::Zero,
        }));

    compiled_function
}

static AVALIBLE_REGISTERS: u64 = 16 - 2;

#[derive(Debug)]
struct Color(u64);

/*
 * Register Allocation
 *
 * Input:
 *      X64SFunction with at most one potential memory op (symbol) per
 *      instruction.
 * Output:
 *      A mapping from Symbols to Option<Color>, None indicates that the
 *      register should be placed on the stack. Some(Color) indicates the
 *      color of the symbol. The color can be in [0,.AVALIBLE_REGISTERS).
 */
fn register_alloc(function: &X64SFunction) -> HashMap<Symbol, Option<Color>> {
    // A default implementation for homework 2 can return a HashMap mapping
    // all the symbols in the function to None.
    let mut map = HashMap::new();

    for assembly in &function.body {
        match assembly {
            X64SAssembly::Instruction(instruction) => {
                match instruction.args {
                    SOperands::One(SOperand::Symbol(symbol)) => {
                        map.insert(symbol, None);
                    }
                    SOperands::One(SOperand::MemorySym(symbol)) => {
                        map.insert(symbol, None);
                    }
                    SOperands::Two(SOperand::Symbol(symbol), _) => {
                        map.insert(symbol, None);
                    }
                    SOperands::Two(SOperand::MemorySym(symbol), _) => {
                        map.insert(symbol, None);
                    }
                    SOperands::Two(_, SOperand::Symbol(symbol)) => {
                        map.insert(symbol, None);
                    }
                    SOperands::Two(_, SOperand::MemorySym(symbol)) => {
                        map.insert(symbol, None);
                    }
                    _ => (),
                };
            }
            _ => (),
        };
    }

    map
}

#[derive(Debug)]
enum StackOrReg {
    Stack(i64),
    Reg(X64Register),
}

/*
 * Register Assignment
 *
 * Input:
 *      The register allocation: a mapping of symbols to optionally a color.
 * Output:
 *      The register assignment: a mapping of symbols to the stack offset or
 *      the register to use.
 */
fn register_assignment(allocation: HashMap<Symbol, Option<Color>>) -> HashMap<Symbol, StackOrReg> {
    // Map all None's to distinct Stack offsets.
    // Map each Color (which contains a value less than AVALIBLE_REGISTERS)
    // to distinct Registers.

    // A default implementation for homework 2 should map all symbols in
    // allocation to some stack offset.
    let mut assignment = HashMap::new();
    let mut offset = -16;

    for (symbol, _) in allocation.iter() {
        assignment.insert(*symbol, StackOrReg::Stack(offset));
        offset -= 8;
    }

    assignment
}

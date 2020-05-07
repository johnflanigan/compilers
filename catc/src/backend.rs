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
    pub label_gen: LabelGenerator,
    pub symbol_gen: SymbolGenerator,
    pub string_literals: HashMap<Label, String>,
}

const QUADWORD_SIZE: i64 = 8;

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
pub fn select(program: LIRProgram, state: &mut GlobalInfo) -> X64SProgram {
    let mut string_literals = HashMap::new();

    let mut selected_program = X64SProgram {
        main_function: select_fn(&program.main_function, &mut string_literals, state),
        other_functions: HashMap::new(),
        string_literals: string_literals,
    };

    for (label, function) in program.other_functions.iter() {
        selected_program.other_functions.insert(
            *label,
            select_fn(function, &mut selected_program.string_literals, state),
        );
    }

    selected_program
}

fn select_fn(
    function: &LIRFunction,
    string_literals: &mut HashMap<Label, String>,
    state: &mut GlobalInfo,
) -> X64SFunction {
    let mut selected_function = X64SFunction { body: Vec::new() };

    // Move registers into parameters
    for (pos, arg) in function.arguments.iter().enumerate() {
        match pos {
            0 => {
                selected_function
                    .body
                    .push(X64SAssembly::Instruction(X64SInstruction {
                        op_code: X64opCode::Movq,
                        args: SOperands::Two(
                            SOperand::Register(X64Register::Rdi),
                            SOperand::Symbol(*arg),
                        ),
                    }));
            }
            1 => {
                selected_function
                    .body
                    .push(X64SAssembly::Instruction(X64SInstruction {
                        op_code: X64opCode::Movq,
                        args: SOperands::Two(
                            SOperand::Register(X64Register::Rsi),
                            SOperand::Symbol(*arg),
                        ),
                    }));
            }
            2 => {
                selected_function
                    .body
                    .push(X64SAssembly::Instruction(X64SInstruction {
                        op_code: X64opCode::Movq,
                        args: SOperands::Two(
                            SOperand::Register(X64Register::Rdx),
                            SOperand::Symbol(*arg),
                        ),
                    }));
            }
            3 => {
                selected_function
                    .body
                    .push(X64SAssembly::Instruction(X64SInstruction {
                        op_code: X64opCode::Movq,
                        args: SOperands::Two(
                            SOperand::Register(X64Register::Rcx),
                            SOperand::Symbol(*arg),
                        ),
                    }));
            }
            4 => {
                selected_function
                    .body
                    .push(X64SAssembly::Instruction(X64SInstruction {
                        op_code: X64opCode::Movq,
                        args: SOperands::Two(
                            SOperand::Register(X64Register::R8),
                            SOperand::Symbol(*arg),
                        ),
                    }));
            }
            5 => {
                selected_function
                    .body
                    .push(X64SAssembly::Instruction(X64SInstruction {
                        op_code: X64opCode::Movq,
                        args: SOperands::Two(
                            SOperand::Register(X64Register::R9),
                            SOperand::Symbol(*arg),
                        ),
                    }));
            }
            _ => {
                panic!("Unexpected number of arguments");
            }
        };
    }

    // Convert LIRAssembly into X64SAssembly
    for assembly in &function.instruction_listing {
        match assembly {
            LIRAssembly::Label(label) => {
                selected_function.body.push(X64SAssembly::Label(*label));
            }
            LIRAssembly::Instruction(instruction) => {
                match instruction {
                    LIRInstruction::Nop => {
                        // Do nothing, move on to next instruction.
                        selected_function
                            .body
                            .push(X64SAssembly::Instruction(X64SInstruction {
                                op_code: X64opCode::Nop,
                                args: SOperands::Zero,
                            }));
                    }
                    LIRInstruction::IntLit { assign_to, value } => {
                        // Mutate the value stored at Symbol to be the same as "value".
                        selected_function
                            .body
                            .push(X64SAssembly::Instruction(X64SInstruction {
                                op_code: X64opCode::Movq,
                                args: SOperands::Two(
                                    SOperand::Immediate(X64Value::Absolute(*value)),
                                    SOperand::Symbol(*assign_to),
                                ),
                            }));
                    }
                    LIRInstruction::StringLit { assign_to, value } => {
                        // Generate new string label
                        let string_label = state.label_gen.new_label();

                        // Add string to string_literals
                        string_literals.insert(string_label, value.to_string());

                        // Mutate the value stored at Symbol to be the memory location of the
                        // immutable string "value".
                        selected_function
                            .body
                            .push(X64SAssembly::Instruction(X64SInstruction {
                                op_code: X64opCode::Lea,
                                args: SOperands::Two(
                                    SOperand::MemoryOffset(
                                        X64Value::LabelRef(string_label),
                                        X64Register::Rip,
                                    ),
                                    SOperand::Register(X64Register::Rax),
                                ),
                            }));

                        // Mutate the value stored at Symbol to be the memory location of the
                        // immutable string "value".
                        selected_function
                            .body
                            .push(X64SAssembly::Instruction(X64SInstruction {
                                op_code: X64opCode::Movq,
                                args: SOperands::Two(
                                    SOperand::Register(X64Register::Rax),
                                    SOperand::Symbol(*assign_to),
                                ),
                            }));
                    }
                    LIRInstruction::StoreToMemoryAtOffset {
                        location,
                        offset,
                        value,
                    } => {
                        let memory_location = state.symbol_gen.new_symbol();

                        // Move 8 into %rax
                        selected_function
                            .body
                            .push(X64SAssembly::Instruction(X64SInstruction {
                                op_code: X64opCode::Movq,
                                args: SOperands::Two(
                                    SOperand::Immediate(X64Value::Absolute(QUADWORD_SIZE)),
                                    SOperand::Register(X64Register::Rax),
                                ),
                            }));

                        // Multiply %rax by offset to get offset in bytes (stored in %rax)
                        selected_function
                            .body
                            .push(X64SAssembly::Instruction(X64SInstruction {
                                op_code: X64opCode::IMulq,
                                args: SOperands::One(SOperand::Symbol(*offset)),
                            }));

                        // Move multiplication result into memory location
                        selected_function
                            .body
                            .push(X64SAssembly::Instruction(X64SInstruction {
                                op_code: X64opCode::Movq,
                                args: SOperands::Two(
                                    SOperand::Register(X64Register::Rax),
                                    SOperand::Symbol(memory_location),
                                ),
                            }));

                        // Add location to offset
                        selected_function
                            .body
                            .push(X64SAssembly::Instruction(X64SInstruction {
                                op_code: X64opCode::Add,
                                args: SOperands::Two(
                                    SOperand::Symbol(*location),
                                    SOperand::Symbol(memory_location),
                                ),
                            }));

                        // Store the value of symbol "value" as the memory location "location + offset".
                        // This is used for array and record mutation.
                        selected_function
                            .body
                            .push(X64SAssembly::Instruction(X64SInstruction {
                                op_code: X64opCode::Movq,
                                args: SOperands::Two(
                                    SOperand::Symbol(*value),
                                    SOperand::MemorySym(memory_location),
                                ),
                            }));
                    }
                    LIRInstruction::LoadFromMemoryAtOffset {
                        assign_to,
                        location,
                        offset,
                    } => {
                        let memory_location = state.symbol_gen.new_symbol();

                        // Move 8 into %rax
                        selected_function
                            .body
                            .push(X64SAssembly::Instruction(X64SInstruction {
                                op_code: X64opCode::Movq,
                                args: SOperands::Two(
                                    SOperand::Immediate(X64Value::Absolute(QUADWORD_SIZE)),
                                    SOperand::Register(X64Register::Rax),
                                ),
                            }));

                        // Multiply %rax by offset to get offset in bytes (stored in %rax)
                        selected_function
                            .body
                            .push(X64SAssembly::Instruction(X64SInstruction {
                                op_code: X64opCode::IMulq,
                                args: SOperands::One(SOperand::Symbol(*offset)),
                            }));

                        // Move multiplication result into memory location
                        selected_function
                            .body
                            .push(X64SAssembly::Instruction(X64SInstruction {
                                op_code: X64opCode::Movq,
                                args: SOperands::Two(
                                    SOperand::Register(X64Register::Rax),
                                    SOperand::Symbol(memory_location),
                                ),
                            }));

                        // Add location to offset
                        selected_function
                            .body
                            .push(X64SAssembly::Instruction(X64SInstruction {
                                op_code: X64opCode::Add,
                                args: SOperands::Two(
                                    SOperand::Symbol(*location),
                                    SOperand::Symbol(memory_location),
                                ),
                            }));

                        // Mutate "assign_to" to be the value stored at the memory location
                        // "location + offset". This is used to read a record or array.
                        selected_function
                            .body
                            .push(X64SAssembly::Instruction(X64SInstruction {
                                op_code: X64opCode::Movq,
                                args: SOperands::Two(
                                    SOperand::MemorySym(memory_location),
                                    SOperand::Symbol(*assign_to),
                                ),
                            }));
                    }
                    LIRInstruction::Assign { assign_to, id } => {
                        // Mutate "assign_to" to be the value in "id".
                        selected_function
                            .body
                            .push(X64SAssembly::Instruction(X64SInstruction {
                                op_code: X64opCode::Movq,
                                args: SOperands::Two(
                                    SOperand::Symbol(*id),
                                    SOperand::Symbol(*assign_to),
                                ),
                            }));
                    }
                    LIRInstruction::Negate { assign_to, value } => {
                        // Mutate "assign_to" to be the value in "id".
                        selected_function
                            .body
                            .push(X64SAssembly::Instruction(X64SInstruction {
                                op_code: X64opCode::Movq,
                                args: SOperands::Two(
                                    SOperand::Symbol(*value),
                                    SOperand::Symbol(*assign_to),
                                ),
                            }));

                        // Mutate "assign_to" to be the negation of "assign_to".
                        selected_function
                            .body
                            .push(X64SAssembly::Instruction(X64SInstruction {
                                op_code: X64opCode::Neg,
                                args: SOperands::One(SOperand::Symbol(*assign_to)),
                            }));
                    }
                    LIRInstruction::BinaryOp {
                        assign_to,
                        left,
                        op,
                        right,
                    } => {
                        // Mutate "assign_to" to be the value in "left".
                        selected_function
                            .body
                            .push(X64SAssembly::Instruction(X64SInstruction {
                                op_code: X64opCode::Movq,
                                args: SOperands::Two(
                                    SOperand::Symbol(*left),
                                    SOperand::Symbol(*assign_to),
                                ),
                            }));

                        match op {
                            InfixOp::Multiply => {
                                selected_function.body.push(X64SAssembly::Instruction(
                                    X64SInstruction {
                                        op_code: X64opCode::Movq,
                                        args: SOperands::Two(
                                            SOperand::Symbol(*left),
                                            SOperand::Register(X64Register::Rax),
                                        ),
                                    },
                                ));

                                selected_function.body.push(X64SAssembly::Instruction(
                                    X64SInstruction {
                                        op_code: X64opCode::IMulq,
                                        args: SOperands::One(SOperand::Symbol(*right)),
                                    },
                                ));

                                selected_function.body.push(X64SAssembly::Instruction(
                                    X64SInstruction {
                                        op_code: X64opCode::Movq,
                                        args: SOperands::Two(
                                            SOperand::Register(X64Register::Rax),
                                            SOperand::Symbol(*assign_to),
                                        ),
                                    },
                                ));
                            }
                            InfixOp::Divide => {
                                selected_function.body.push(X64SAssembly::Instruction(
                                    X64SInstruction {
                                        op_code: X64opCode::Movq,
                                        args: SOperands::Two(
                                            SOperand::Symbol(*left),
                                            SOperand::Register(X64Register::Rax),
                                        ),
                                    },
                                ));

                                selected_function.body.push(X64SAssembly::Instruction(
                                    X64SInstruction {
                                        op_code: X64opCode::IDivq,
                                        args: SOperands::One(SOperand::Symbol(*right)),
                                    },
                                ));

                                selected_function.body.push(X64SAssembly::Instruction(
                                    X64SInstruction {
                                        op_code: X64opCode::Movq,
                                        args: SOperands::Two(
                                            SOperand::Register(X64Register::Rax),
                                            SOperand::Symbol(*assign_to),
                                        ),
                                    },
                                ));
                            }
                            InfixOp::Add => {
                                selected_function.body.push(X64SAssembly::Instruction(
                                    X64SInstruction {
                                        op_code: X64opCode::Movq,
                                        args: SOperands::Two(
                                            SOperand::Symbol(*left),
                                            SOperand::Symbol(*assign_to),
                                        ),
                                    },
                                ));

                                selected_function.body.push(X64SAssembly::Instruction(
                                    X64SInstruction {
                                        op_code: X64opCode::Add,
                                        args: SOperands::Two(
                                            SOperand::Symbol(*right),
                                            SOperand::Symbol(*assign_to),
                                        ),
                                    },
                                ));
                            }
                            InfixOp::Subtract => {
                                selected_function.body.push(X64SAssembly::Instruction(
                                    X64SInstruction {
                                        op_code: X64opCode::Movq,
                                        args: SOperands::Two(
                                            SOperand::Symbol(*left),
                                            SOperand::Symbol(*assign_to),
                                        ),
                                    },
                                ));

                                selected_function.body.push(X64SAssembly::Instruction(
                                    X64SInstruction {
                                        op_code: X64opCode::Sub,
                                        args: SOperands::Two(
                                            SOperand::Symbol(*right),
                                            SOperand::Symbol(*assign_to),
                                        ),
                                    },
                                ));
                            }
                            InfixOp::And => {
                                selected_function.body.push(X64SAssembly::Instruction(
                                    X64SInstruction {
                                        op_code: X64opCode::Movq,
                                        args: SOperands::Two(
                                            SOperand::Symbol(*left),
                                            SOperand::Symbol(*assign_to),
                                        ),
                                    },
                                ));

                                selected_function.body.push(X64SAssembly::Instruction(
                                    X64SInstruction {
                                        op_code: X64opCode::And,
                                        args: SOperands::Two(
                                            SOperand::Symbol(*right),
                                            SOperand::Symbol(*assign_to),
                                        ),
                                    },
                                ));
                            }
                            InfixOp::Or => {
                                selected_function.body.push(X64SAssembly::Instruction(
                                    X64SInstruction {
                                        op_code: X64opCode::Movq,
                                        args: SOperands::Two(
                                            SOperand::Symbol(*left),
                                            SOperand::Symbol(*assign_to),
                                        ),
                                    },
                                ));

                                selected_function.body.push(X64SAssembly::Instruction(
                                    X64SInstruction {
                                        op_code: X64opCode::Or,
                                        args: SOperands::Two(
                                            SOperand::Symbol(*right),
                                            SOperand::Symbol(*assign_to),
                                        ),
                                    },
                                ));
                            }
                        }
                    }
                    LIRInstruction::Call {
                        assign_to,
                        function_name,
                        args,
                    } => {
                        for (pos, arg) in args.iter().enumerate() {
                            match pos {
                                0 => {
                                    selected_function.body.push(X64SAssembly::Instruction(
                                        X64SInstruction {
                                            op_code: X64opCode::Movq,
                                            args: SOperands::Two(
                                                SOperand::Symbol(*arg),
                                                SOperand::Register(X64Register::Rdi),
                                            ),
                                        },
                                    ));
                                }
                                1 => {
                                    selected_function.body.push(X64SAssembly::Instruction(
                                        X64SInstruction {
                                            op_code: X64opCode::Movq,
                                            args: SOperands::Two(
                                                SOperand::Symbol(*arg),
                                                SOperand::Register(X64Register::Rsi),
                                            ),
                                        },
                                    ));
                                }
                                2 => {
                                    selected_function.body.push(X64SAssembly::Instruction(
                                        X64SInstruction {
                                            op_code: X64opCode::Movq,
                                            args: SOperands::Two(
                                                SOperand::Symbol(*arg),
                                                SOperand::Register(X64Register::Rdx),
                                            ),
                                        },
                                    ));
                                }
                                3 => {
                                    selected_function.body.push(X64SAssembly::Instruction(
                                        X64SInstruction {
                                            op_code: X64opCode::Movq,
                                            args: SOperands::Two(
                                                SOperand::Symbol(*arg),
                                                SOperand::Register(X64Register::Rcx),
                                            ),
                                        },
                                    ));
                                }
                                4 => {
                                    selected_function.body.push(X64SAssembly::Instruction(
                                        X64SInstruction {
                                            op_code: X64opCode::Movq,
                                            args: SOperands::Two(
                                                SOperand::Symbol(*arg),
                                                SOperand::Register(X64Register::R8),
                                            ),
                                        },
                                    ));
                                }
                                5 => {
                                    selected_function.body.push(X64SAssembly::Instruction(
                                        X64SInstruction {
                                            op_code: X64opCode::Movq,
                                            args: SOperands::Two(
                                                SOperand::Symbol(*arg),
                                                SOperand::Register(X64Register::R9),
                                            ),
                                        },
                                    ));
                                }
                                _ => {
                                    panic!("Unexpected number of args");
                                }
                            };
                        }

                        selected_function
                            .body
                            .push(X64SAssembly::Instruction(X64SInstruction {
                                op_code: X64opCode::Call,
                                args: SOperands::One(SOperand::MemoryImm(X64Value::LabelRef(
                                    *function_name,
                                ))),
                            }));

                        selected_function
                            .body
                            .push(X64SAssembly::Instruction(X64SInstruction {
                                op_code: X64opCode::Movq,
                                args: SOperands::Two(
                                    SOperand::Register(X64Register::Rax),
                                    SOperand::Symbol(*assign_to),
                                ),
                            }));
                    }
                    LIRInstruction::Jump { to } => {
                        selected_function
                            .body
                            .push(X64SAssembly::Instruction(X64SInstruction {
                                op_code: X64opCode::Jmp,
                                args: SOperands::One(SOperand::MemoryImm(X64Value::LabelRef(*to))),
                            }));
                    }
                    LIRInstruction::JumpC { to, condition } => {
                        // Perform comparison, swapping arguments to match assembly expectations
                        selected_function
                            .body
                            .push(X64SAssembly::Instruction(X64SInstruction {
                                op_code: X64opCode::Cmp,
                                args: SOperands::Two(
                                    SOperand::Symbol(condition.right),
                                    SOperand::Symbol(condition.left),
                                ),
                            }));

                        // Jump based on condition
                        match condition.c {
                            ComparisonType::Equal => {
                                selected_function.body.push(X64SAssembly::Instruction(
                                    X64SInstruction {
                                        op_code: X64opCode::Je,
                                        args: SOperands::One(SOperand::MemoryImm(
                                            X64Value::LabelRef(*to),
                                        )),
                                    },
                                ));
                            }
                            ComparisonType::NotEqual => {
                                selected_function.body.push(X64SAssembly::Instruction(
                                    X64SInstruction {
                                        op_code: X64opCode::Jne,
                                        args: SOperands::One(SOperand::MemoryImm(
                                            X64Value::LabelRef(*to),
                                        )),
                                    },
                                ));
                            }
                            ComparisonType::GreaterThan => {
                                selected_function.body.push(X64SAssembly::Instruction(
                                    X64SInstruction {
                                        op_code: X64opCode::Jg,
                                        args: SOperands::One(SOperand::MemoryImm(
                                            X64Value::LabelRef(*to),
                                        )),
                                    },
                                ));
                            }
                            ComparisonType::LessThan => {
                                selected_function.body.push(X64SAssembly::Instruction(
                                    X64SInstruction {
                                        op_code: X64opCode::Jl,
                                        args: SOperands::One(SOperand::MemoryImm(
                                            X64Value::LabelRef(*to),
                                        )),
                                    },
                                ));
                            }
                            ComparisonType::GreaterThanEqual => {
                                selected_function.body.push(X64SAssembly::Instruction(
                                    X64SInstruction {
                                        op_code: X64opCode::Jge,
                                        args: SOperands::One(SOperand::MemoryImm(
                                            X64Value::LabelRef(*to),
                                        )),
                                    },
                                ));
                            }
                            ComparisonType::LessThanEqual => {
                                selected_function.body.push(X64SAssembly::Instruction(
                                    X64SInstruction {
                                        op_code: X64opCode::Jle,
                                        args: SOperands::One(SOperand::MemoryImm(
                                            X64Value::LabelRef(*to),
                                        )),
                                    },
                                ));
                            }
                        }
                    }
                }
            }
        }
    }

    // Move return value into register
    selected_function
        .body
        .push(X64SAssembly::Instruction(X64SInstruction {
            op_code: X64opCode::Movq,
            args: SOperands::Two(
                SOperand::Symbol(function.return_symbol),
                SOperand::Register(X64Register::Rax),
            ),
        }));

    return selected_function;
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
                    SOperands::Two(SOperand::Symbol(x), SOperand::Symbol(y)) => {
                        // mov 'x' into %rax
                        let instruction_1 = X64SInstruction {
                            op_code: X64opCode::Movq,
                            args: SOperands::Two(
                                SOperand::Symbol(x),
                                SOperand::Register(X64Register::Rax),
                            ),
                        };
                        fixed_function
                            .body
                            .push(X64SAssembly::Instruction(instruction_1));

                        // opcode %rax, 'y'
                        let instruction_2 = X64SInstruction {
                            op_code: instruction.op_code,
                            args: SOperands::Two(
                                SOperand::Register(X64Register::Rax),
                                SOperand::Symbol(y),
                            ),
                        };
                        fixed_function
                            .body
                            .push(X64SAssembly::Instruction(instruction_2));
                    }
                    // op ('p'), %reg
                    SOperands::Two(SOperand::MemorySym(p), SOperand::Register(reg)) => {
                        // mov 'p' into %rax
                        let instruction_1 = X64SInstruction {
                            op_code: X64opCode::Movq,
                            args: SOperands::Two(
                                SOperand::Symbol(p),
                                SOperand::Register(X64Register::Rax),
                            ),
                        };
                        fixed_function
                            .body
                            .push(X64SAssembly::Instruction(instruction_1));

                        // mov (%rax) into %rax
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

                        // opcode %rax, %reg
                        let instruction_3 = X64SInstruction {
                            op_code: instruction.op_code,
                            args: SOperands::Two(
                                SOperand::Register(X64Register::Rax),
                                SOperand::Register(reg),
                            ),
                        };
                        fixed_function
                            .body
                            .push(X64SAssembly::Instruction(instruction_3));
                    }
                    // op ('p'), 'y'
                    SOperands::Two(SOperand::MemorySym(p), SOperand::Symbol(y)) => {
                        // mov 'p' into %rax
                        let instruction_1 = X64SInstruction {
                            op_code: X64opCode::Movq,
                            args: SOperands::Two(
                                SOperand::Symbol(p),
                                SOperand::Register(X64Register::Rax),
                            ),
                        };
                        fixed_function
                            .body
                            .push(X64SAssembly::Instruction(instruction_1));

                        // mov (%rax) into %rax
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

                        // opcode %rax, 'y'
                        let instruction_3 = X64SInstruction {
                            op_code: instruction.op_code,
                            args: SOperands::Two(
                                SOperand::Register(X64Register::Rax),
                                SOperand::Symbol(y),
                            ),
                        };
                        fixed_function
                            .body
                            .push(X64SAssembly::Instruction(instruction_3));
                    }
                    // op ('p'), ('q')
                    SOperands::Two(SOperand::MemorySym(p), SOperand::MemorySym(q)) => {
                        // mov p into %rax
                        let instruction_1 = X64SInstruction {
                            op_code: X64opCode::Movq,
                            args: SOperands::Two(
                                SOperand::Symbol(p),
                                SOperand::Register(X64Register::Rax),
                            ),
                        };
                        fixed_function
                            .body
                            .push(X64SAssembly::Instruction(instruction_1));

                        // mov (%rax) into %rax
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

                        // mov 'q' into %rdx
                        let instruction_3 = X64SInstruction {
                            op_code: X64opCode::Movq,
                            args: SOperands::Two(
                                SOperand::Symbol(q),
                                SOperand::Register(X64Register::Rdx),
                            ),
                        };
                        fixed_function
                            .body
                            .push(X64SAssembly::Instruction(instruction_3));

                        // opcode %rax, (%rdx)
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
                    // op 'x', ('q')
                    SOperands::Two(SOperand::Symbol(x), SOperand::MemorySym(q)) => {
                        // mov 'x' into %rax
                        let instruction_1 = X64SInstruction {
                            op_code: X64opCode::Movq,
                            args: SOperands::Two(
                                SOperand::Symbol(x),
                                SOperand::Register(X64Register::Rax),
                            ),
                        };
                        fixed_function
                            .body
                            .push(X64SAssembly::Instruction(instruction_1));

                        // mov 'q' into %rdx
                        let instruction_2 = X64SInstruction {
                            op_code: X64opCode::Movq,
                            args: SOperands::Two(
                                SOperand::Symbol(q),
                                SOperand::Register(X64Register::Rdx),
                            ),
                        };
                        fixed_function
                            .body
                            .push(X64SAssembly::Instruction(instruction_2));

                        // opcode %rax, (%rdx)
                        let instruction_3 = X64SInstruction {
                            op_code: instruction.op_code,
                            args: SOperands::Two(
                                SOperand::Register(X64Register::Rax),
                                SOperand::MemoryReg(X64Register::Rdx),
                            ),
                        };
                        fixed_function
                            .body
                            .push(X64SAssembly::Instruction(instruction_3));
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

    // Ensure allocation is multiple of 16
    let homes_len: i64 = homes.len().try_into().unwrap();
    let reservations: i64 = if homes.len() % 2 == 0 {
        homes_len
    } else {
        homes_len + 1
    };
    let stack_reservation: i64 = reservations * QUADWORD_SIZE;
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
    let mut offset = -QUADWORD_SIZE;

    for (symbol, _) in allocation.iter() {
        assignment.insert(*symbol, StackOrReg::Stack(offset));
        offset -= QUADWORD_SIZE;
    }

    assignment
}

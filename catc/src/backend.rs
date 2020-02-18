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
 *          symbol generator that generates new, valid labels for the program
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
 *      XS64Program with at most one potential memory op (symbol) per
 *      instruction.
 */
pub fn fix_up(progam: X64SProgram) -> X64SProgram {
    unimplemented!("Homework 2");
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
    unimplemented!("Homework 2");
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
    unimplemented!("Homework 2");
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
fn register_alloc(progam: &X64SFunction) -> HashMap<Symbol, Option<Color>> {
    // A default implementation for homework 2 can return a HashMap mapping
    // all the symbols in the function to None.
    unimplemented!("Homework N");
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
    unimplemented!("Homework N");
}

use crate::checked_grammar::*;
use crate::common::*;
use crate::lir::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct LoweringGlobal {
    pub main_function: LIRFunction,
    pub other_functions: HashMap<Label, LIRFunction>,
    // Track what function or loop scope we are in
}

#[allow(dead_code, unused_variables)]
pub fn lower(type_checked_program: CheckedProgram) -> LIRProgram {
    for checked_top_level_dec in type_checked_program.dec_list {
        // Create LIR function
        let lir_function = LIRFunction {
            locals: vec![],
            arguments: vec![],
            return_symbol: None,
            instruction_listing: vec![],
        };

        match checked_top_level_dec {
            CheckedTopLevelDec::FunDec { name, args, body } => {
                // Lower expression
                let (lir_assembly, symbol) = lower_exp(checked_exp: body);

                // Create LIR function

                // Update LIR program
            }
        }
    }

    // Return LIR program
}

// Returns a sequence of LIR instructions and the symbol that will hold the result of those computations
fn lower_exp(checked_exp: CheckedExp) -> (Vec<LIRAssembly>, Symbol) {
    match checked_exp {
        CheckedExp::Break => {
            unimplemented!();
        }
        CheckedExp::IntLit { value } => {
            // Create temporary symbol
            let symbol = type_checked_program.gen_sym.new_symbol();
            // Assign Int to temporary symbol
            let instruction = LIRInstruction::IntLit {
                assign_to: symbol,
                value: value,
            };
            let assembly = LIRAssembly::Instruction { instruction };
            // Return assembly instruction and temporary symbol
            (!vec[assembly], symbol)
        }
        CheckedExp::StringLit { value } => {
            // Create temporary symbol
            let symbol = type_checked_program.gen_sym.new_symbol();
            // Assign String to temporary symbol
            let instruction = LIRInstruction::StringLit {
                assign_to: symbol,
                value: value,
            };
            let assembly = LIRAssembly::Instruction { instruction };
            // Return assembly instruction and temporary symbol
            (!vec[assembly], symbol)
        }
        CheckedExp::LValue { lvalue } => match lvalue {
            // These cases should be treated as loading values into temporary symbols
            CheckedLValue::Id { name } => {
                // Create temporary symbol
                let symbol = type_checked_program.gen_sym.new_symbol();
                // Assign Id to temporary symbol
                let instruction = LIRInstruction::Assign {
                    assign_to: symbol,
                    id: name,
                };
                let assembly = LIRAssembly::Instruction { instruction };
                // Return assembly instruction and temporary symbol
                (!vec[assembly], symbol)
            }
            CheckedLValue::Subscript { array, index } => {
                unimplemented!();
            }
            CheckedLValue::FieldExp { record, field } => {
                unimplemented!();
            }
        },
        CheckedExp::Sequence { sequence } => {
            unimplemented!();
        }
        CheckedExp::Negate { exp } => {
            let (lir_assembly, symbol) = lower_exp(exp);

            // Create temporary symbol
            let temp_symbol = type_checked_program.gen_sym.new_symbol();

            // Negate returned symbol
            let instruction = LIRInstruction::Negate {
                assign_to: temp_symbol,
                value: symbol,
            };

            let assembly = LIRAssembly::Instruction { instruction };
            lir_assembly.push(assembly);

            (lir_assembly, temp_symbol)
        }
        CheckedExp::Infix { left, op, right } => {
            // Call lower_exp on the left-hand operand to get sequence1 and symbol1
            let (lir_assembly_1, symbol_1) = lower_exp(left);
            // Call lower_exp on the right-hand operand to get sequence2 and symbol2
            let (lir_assembly_2, symbol_2) = lower_exp(right);

            // Generate a new temp symbol3
            let symbol_3 = type_checked_program.gen_sym.new_symbol();

            // Concatenate sequence1 + sequence2 + BinaryOp(symbol3, symbol1, Add, symbol2)
            let instruction = LIRInstruction::BinaryOp {
                assign_to: symbol_3,
                left: symbol_1,
                op: op,
                right: symbol_2,
            };
            let assembly = LIRAssembly::Instruction { instruction };

            let lir_assembly_3 = vec![];
            lir_assembly_3
                .append(lir_assembly_1)
                .append(lir_assembly_2)
                .append(assembly);

            // Return the new sequence and symbol3
            (lir_assembly_3, symbol_3)
        }
        CheckedExp::ArrayCreate {
            length,
            initial_value,
        } => {
            unimplemented!();
        }
        CheckedExp::RecordCreate { fields } => {
            unimplemented!();
        }
        CheckedExp::Assign { left, right } => {
            unimplemented!();
        }
        CheckedExp::IfThenElse {
            if_exp,
            then_exp,
            else_exp,
        } => {}
        CheckedExp::IfThen { if_exp, then_exp } => {
            unimplemented!();
        }
        CheckedExp::While { while_exp, do_exp } => {
            unimplemented!();
        }
        CheckedExp::For {
            id,
            for_exp,
            to_exp,
            do_exp,
        } => {
            unimplemented!();
        }
        CheckedExp::Let { let_exp, in_exp } => {
            unimplemented!();
        }
        CheckedExp::Call {
            function_name,
            args,
        } => {
            unimplemented!();
        }
    }
}

fn lower_lvalue_value(
    checked: CheckedLValue,
    assign_to: Symbol,
    break_to: Option<Label>,
    gen: &mut LoweringGlobal,
) -> (Type, Vec<LIRAssembly>) {
    unimplemented!();
}

fn lower_lvalue_assigning_symbol(
    checked: CheckedLValue,
    symbol: Symbol,
    break_to: Option<Label>,
    gen: &mut LoweringGlobal,
) -> Vec<LIRAssembly> {
    unimplemented!();
}

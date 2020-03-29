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
            let int_lit_symbol = type_checked_program.gen_sym.new_symbol();
            // Assign Int to temporary symbol
            let int_lit_instruction = LIRInstruction::IntLit {
                assign_to: int_lit_symbol,
                value: value,
            };
            let int_lit_assembly = LIRAssembly::Instruction {
                int_lit_instruction,
            };
            // Return assembly instruction and temporary symbol
            (!vec[int_lit_assembly], int_lit_symbol)
        }
        CheckedExp::StringLit { value } => {
            // Create temporary symbol
            let string_symbol = type_checked_program.gen_sym.new_symbol();
            // Assign String to temporary symbol
            let string_instruction = LIRInstruction::StringLit {
                assign_to: string_symbol,
                value: value,
            };
            let string_assembly = LIRAssembly::Instruction { string_instruction };
            // Return assembly instruction and temporary symbol
            (!vec[string_assembly], string_symbol)
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
            let (exp_assembly, exp_symbol) = lower_exp(exp);

            // Create temporary symbol
            let negate_symbol = type_checked_program.gen_sym.new_symbol();

            // Negate returned symbol
            let lir_negate_instruction = LIRInstruction::Negate {
                assign_to: temp_symbol,
                value: symbol,
            };
            let lir_negate_assembly = LIRAssembly::Instruction {
                lir_negate_instruction,
            };

            let negate_assembly = vec![];
            negate_assembly
                .append(exp_assembly)
                .append(lir_negate_assembly);

            (negate_assembly, negate_symbol)
        }
        CheckedExp::Infix { left, op, right } => {
            // Call lower_exp on the left-hand operand to get left_assembly and left_symbol
            let (left_assembly, left_symbol) = lower_exp(left);
            // Call lower_exp on the right-hand operand to get right_assembly and right_symbol
            let (right_assembly, right_symbol) = lower_exp(right);

            // Generate a new temp symbol3
            let infix_symbol = type_checked_program.gen_sym.new_symbol();

            // Concatenate sequence1 + sequence2 + BinaryOp(symbol3, symbol1, Add, symbol2)
            let binary_op_instruction = LIRInstruction::BinaryOp {
                assign_to: infix_symbol,
                left: left_symbol,
                op: op,
                right: right_symbol,
            };
            let binary_op_assembly = LIRAssembly::Instruction {
                binary_op_instruction,
            };

            let infix_assembly = vec![];
            infix_assembly
                .append(left_assembly)
                .append(right_assembly)
                .append(binary_op_assembly);

            // Return the new sequence and symbol3
            (infix_assembly, infix_symbol)
        }
        CheckedExp::ArrayCreate {
            length,
            initial_value,
        } => {
            // Create new symbol
            let array_symbol = type_checked_program.gen_sym.new_symbol();

            // Load length and initial_value expressions into temporary symbols
            let (length_assembly, length_symbol) = lower_exp(length);
            let (initial_value_assembly, initial_value_symbol) = lower_exp(initial_value);

            // Call allocate_and_memset and set result to new symbol
            let call_instruction = LIRInstruction::Call {
                assign_to: array_symbol,
                function_name: Label::AllocateAndMemset,
                args: vec![length_symbol, initial_value_symbol],
            };
            let call_assembly = LIRAssembly::Instruction { call_instruction };

            // Append call instruction
            let array_assembly = vec![];
            array_assembly
                .append(length_assembly)
                .append(initial_value_assembly)
                .append(call_assembly);

            // Return the result
            (array_assembly, array_symbol)
        }
        CheckedExp::RecordCreate { fields } => {
            // Create new symbol for record
            let record_symbol = type_checked_program.gen_sym.new_symbol();

            // Allocate length of fields vector and set record to address
            let call_instruction = LIRInstruction::Call {
                assign_to: record_symbol,
                function_name: Label::AllocateAndMemset,
                args: vec![fields.len()],
            };
            let call_assembly = LIRAssembly::Instruction { call_instruction };

            let record_assembly = vec![call_assembly];

            for (pos, (_, exp)) in fields.iter().enumerate() {
                let (exp_assembly, exp_symbol) = lower_exp(exp);
                record_assembly.append(exp_assembly);

                // Geneate offset symbol
                let offset_symbol = type_checked_program.gen_sym.new_symbol();

                // Set offset equal to position
                let offset_instruction = LIRInstruction::IntLit {
                    location: offset_symbol,
                    offset: pos,
                };
                let offset_assembly = LIRAssembly::Instruction { offset_instruction };
                record_assembly.push(offset_assembly);

                // Set record field to exp
                let store_to_memory_at_offset_instruction = LIRInstruction::StoreToMemoryAtOffset {
                    location: record_symbol,
                    offset: pos,
                    value: exp_symbol,
                };
                let store_to_memory_at_offset_assembly = LIRAssembly::Instruction {
                    store_to_memory_at_offset_instruction,
                };
                record_assembly.push(store_to_memory_at_offset_assembly);
            }

            (record_assembly, record_symbol)
        }
        CheckedExp::Assign { left, right } => {
            // Call lower_exp on the left-hand operand to get left_assembly and left_symbol
            let (left_assembly, left_symbol) = lower_exp(left);

            // Call lower_exp on the right-hand operand to get right_assembly and right_symbol
            let (right_assembly, right_symbol) = lower_exp(right);

            let lir_assign_instruction = LIRInstruction::Assign {
                assign_to: left_symbol,
                id: right_symbol,
            };
            let lir_assign_assembly = LIRAssembly::Instruction {
                lir_assign_instruction,
            };

            let assign_assembly = vec![];
            assign_assembly
                .append(left_assembly)
                .append(right_assembly)
                .push(lir_assign_assembly);

            (assign_assembly, left_symbol)
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
            // TODO unclear what let does in language
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

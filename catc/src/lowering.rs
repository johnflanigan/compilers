use crate::checked_grammar::*;
use crate::common::*;
use crate::lir::*;
use std::collections::HashMap;
use std::convert::TryInto;

#[derive(Debug)]
pub struct LoweringGlobal {
    pub gen_sym: SymbolGenerator,
    pub gen_label: LabelGenerator,
}

#[allow(dead_code, unused_variables)]
pub fn lower(type_checked_program: CheckedProgram) -> LIRProgram {
    // for checked_top_level_dec in type_checked_program.dec_list {
    //     // Create LIR function
    //     let lir_function = LIRFunction {
    //         locals: vec![],
    //         arguments: vec![],
    //         return_symbol: None,
    //         instruction_listing: vec![],
    //     };

    //     match checked_top_level_dec {
    //         CheckedTopLevelDec::FunDec { name, args, body } => {
    //             // Lower expression
    //             let (lir_assembly, symbol) = lower_exp(checked_exp: body);

    //             // Create LIR function

    //             // Update LIR program
    //         }
    //     }
    // }

    // Return LIR program
    unimplemented!()
}

// Returns a sequence of LIR instructions and the symbol that will hold the result of those computations
fn lower_exp(
    checked_exp: CheckedExp,
    lowering_global: LoweringGlobal,
) -> (Vec<LIRAssembly>, Symbol) {
    match checked_exp {
        CheckedExp::Break => {
            unimplemented!();
        }
        CheckedExp::IntLit { value } => {
            // Create temporary symbol
            let int_lit_symbol = lowering_global.gen_sym.new_symbol();
            // Assign Int to temporary symbol
            let int_lit_instruction = LIRInstruction::IntLit {
                assign_to: int_lit_symbol,
                value: value as i64,
            };
            let int_lit_assembly = LIRAssembly::Instruction(int_lit_instruction);
            // Return assembly instruction and temporary symbol
            (vec![int_lit_assembly], int_lit_symbol)
        }
        CheckedExp::StringLit { value } => {
            // Create temporary symbol
            let string_symbol = lowering_global.gen_sym.new_symbol();
            // Assign String to temporary symbol
            let string_instruction = LIRInstruction::StringLit {
                assign_to: string_symbol,
                value: value,
            };
            let string_assembly = LIRAssembly::Instruction(string_instruction);
            // Return assembly instruction and temporary symbol
            (vec![string_assembly], string_symbol)
        }
        CheckedExp::LValue { lvalue } => match lvalue {
            // These cases should be treated as loading values into temporary symbols
            CheckedLValue::Id { name } => {
                // Create temporary symbol
                let symbol = lowering_global.gen_sym.new_symbol();
                // Assign Id to temporary symbol
                let instruction = LIRInstruction::Assign {
                    assign_to: symbol,
                    id: name,
                };
                let assembly = LIRAssembly::Instruction(instruction);
                // Return assembly instruction and temporary symbol
                (vec![assembly], symbol)
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
            let (exp_assembly, exp_symbol) = lower_exp(*exp, lowering_global);

            // Create temporary symbol
            let negate_symbol = lowering_global.gen_sym.new_symbol();

            // Negate returned symbol
            let lir_negate_instruction = LIRInstruction::Negate {
                assign_to: negate_symbol,
                value: exp_symbol,
            };
            let lir_negate_assembly = LIRAssembly::Instruction(lir_negate_instruction);

            let negate_assembly = vec![];
            negate_assembly.append(&mut exp_assembly);
            negate_assembly.push(lir_negate_assembly);

            (negate_assembly, negate_symbol)
        }
        CheckedExp::Infix { left, op, right } => {
            // Call lower_exp on the left-hand operand to get left_assembly and left_symbol
            let (left_assembly, left_symbol) = lower_exp(*left, lowering_global);
            // Call lower_exp on the right-hand operand to get right_assembly and right_symbol
            let (right_assembly, right_symbol) = lower_exp(*right, lowering_global);

            // Generate a new temp symbol3
            let infix_symbol = lowering_global.gen_sym.new_symbol();

            // Concatenate sequence1 + sequence2 + BinaryOp(symbol3, symbol1, Add, symbol2)
            let binary_op_instruction = LIRInstruction::BinaryOp {
                assign_to: infix_symbol,
                left: left_symbol,
                op: op.try_into().unwrap(),
                right: right_symbol,
            };
            let binary_op_assembly = LIRAssembly::Instruction(binary_op_instruction);

            let infix_assembly = vec![];
            infix_assembly.append(&mut left_assembly);
            infix_assembly.append(&mut right_assembly);
            infix_assembly.push(binary_op_assembly);

            // Return the new sequence and symbol3
            (infix_assembly, infix_symbol)
        }
        CheckedExp::ArrayCreate {
            length,
            initial_value,
        } => {
            // Create new symbol
            let array_symbol = lowering_global.gen_sym.new_symbol();

            // Load length and initial_value expressions into temporary symbols
            let (length_assembly, length_symbol) = lower_exp(*length, lowering_global);
            let (initial_value_assembly, initial_value_symbol) =
                lower_exp(*initial_value, lowering_global);

            // Call allocate_and_memset and set result to new symbol
            let call_instruction = LIRInstruction::Call {
                assign_to: array_symbol,
                function_name: Label::AllocateAndMemset,
                args: vec![length_symbol, initial_value_symbol],
            };
            let call_assembly = LIRAssembly::Instruction(call_instruction);

            // Append call instruction
            let array_assembly = vec![];
            array_assembly.append(&mut length_assembly);
            array_assembly.append(&mut initial_value_assembly);
            array_assembly.push(call_assembly);

            // Return the result
            (array_assembly, array_symbol)
        }
        CheckedExp::RecordCreate { fields } => {
            // Create new symbol for record
            let record_symbol = lowering_global.gen_sym.new_symbol();

            let record_assembly = vec![];

            // Geneate length symbol
            let length_symbol = lowering_global.gen_sym.new_symbol();

            // Set length equal to length of fields vector
            let length_instruction = LIRInstruction::IntLit {
                assign_to: length_symbol,
                value: fields.len() as i64,
            };
            let length_assembly = LIRAssembly::Instruction(length_instruction);
            record_assembly.push(length_assembly);

            // Allocate length of fields vector and set record to address
            let call_instruction = LIRInstruction::Call {
                assign_to: record_symbol,
                function_name: Label::AllocateAndMemset,
                args: vec![length_symbol],
            };
            let call_assembly = LIRAssembly::Instruction(call_instruction);
            record_assembly.push(call_assembly);

            for (pos, (_, exp)) in fields.iter().enumerate() {
                let (exp_assembly, exp_symbol) = lower_exp(*exp, lowering_global);
                record_assembly.append(&mut exp_assembly);

                // Geneate offset symbol
                let offset_symbol = lowering_global.gen_sym.new_symbol();

                // Set offset equal to position
                let offset_instruction = LIRInstruction::IntLit {
                    assign_to: offset_symbol,
                    value: pos as i64,
                };
                let offset_assembly = LIRAssembly::Instruction(offset_instruction);
                record_assembly.push(offset_assembly);

                // Set record field to exp
                let store_to_memory_at_offset_instruction = LIRInstruction::StoreToMemoryAtOffset {
                    location: record_symbol,
                    offset: offset_symbol,
                    value: exp_symbol,
                };
                let store_to_memory_at_offset_assembly =
                    LIRAssembly::Instruction(store_to_memory_at_offset_instruction);
                record_assembly.push(store_to_memory_at_offset_assembly);
            }

            (record_assembly, record_symbol)
        }
        CheckedExp::Assign { left, right } => {
            // Call lower_exp on the left-hand operand to get left_assembly and left_symbol
            let (left_assembly, left_symbol) = lower_exp(left, lowering_global);

            // Call lower_exp on the right-hand operand to get right_assembly and right_symbol
            let (right_assembly, right_symbol) = lower_exp(*right, lowering_global);

            let lir_assign_instruction = LIRInstruction::Assign {
                assign_to: left_symbol,
                id: right_symbol,
            };
            let lir_assign_assembly = LIRAssembly::Instruction(lir_assign_instruction);

            let assign_assembly = vec![];
            assign_assembly.append(&mut left_assembly);
            assign_assembly.append(&mut right_assembly);
            assign_assembly.push(lir_assign_assembly);

            (assign_assembly, left_symbol)
        }
        CheckedExp::IfThenElse {
            if_exp,
            then_exp,
            else_exp,
        } => {
            let if_then_else_assembly = vec![];

            // Generate true, false, and end labels
            let true_label = lowering_global.gen_label.new_label();
            let false_label = lowering_global.gen_label.new_label();
            let end_label = lowering_global.gen_label.new_label();

            // Lower condition
            let (if_assembly, if_symbol) = lower_exp(*if_exp, lowering_global);
            if_then_else_assembly.append(&mut if_assembly);

            // Create zero for use in comparison
            let zero_symbol = lowering_global.gen_sym.new_symbol();
            let zero_instruction = LIRInstruction::IntLit {
                assign_to: zero_symbol,
                value: 0,
            };
            let zero_assembly = LIRAssembly::Instruction(zero_instruction);
            if_then_else_assembly.push(zero_assembly);

            // Create not zero comparison
            let not_zero_comparison = Comparison {
                c: ComparisonType::NotEqual,
                left: if_symbol,
                right: zero_symbol,
            };

            // Jump true conditional
            let jump_true_instruction = LIRInstruction::JumpC {
                to: true_label,
                condition: not_zero_comparison,
            };
            let jump_true_assembly = LIRAssembly::Instruction(jump_true_instruction);
            if_then_else_assembly.push(jump_true_assembly);

            // Create zero comparison
            let zero_comparison = Comparison {
                c: ComparisonType::NotEqual,
                left: if_symbol,
                right: zero_symbol,
            };

            // Jump false conditional
            let jump_false_instruction = LIRInstruction::JumpC {
                to: false_label,
                condition: zero_comparison,
            };
            let jump_false_assembly = LIRAssembly::Instruction(jump_false_instruction);
            if_then_else_assembly.push(jump_false_assembly);

            // Emit true label
            let true_label_assembly = LIRAssembly::Label(true_label);
            if_then_else_assembly.push(true_label_assembly);

            // Lower then branch
            let (then_assembly, then_symbol) = lower_exp(*then_exp, lowering_global);
            if_then_else_assembly.append(&mut then_assembly);

            // Jump end label
            let jump_instruction = LIRInstruction::Jump { to: end_label };
            let jump_assembly = LIRAssembly::Instruction(jump_instruction);
            if_then_else_assembly.push(jump_assembly);

            // Emit false label
            let false_label_assembly = LIRAssembly::Label(false_label);
            if_then_else_assembly.push(false_label_assembly);

            // Lower else branch
            let (else_assembly, else_symbol) = lower_exp(*else_exp, lowering_global);
            if_then_else_assembly.append(&mut else_assembly);

            // Emit end label
            let end_label_assembly = LIRAssembly::Label(end_label);
            if_then_else_assembly.push(end_label_assembly);

            // TODO unsure what to return here as a symbol
            (if_then_else_assembly, then_symbol)
        }
        CheckedExp::IfThen { if_exp, then_exp } => {
            let if_then_assembly = vec![];

            // Generate end label
            let end_label = lowering_global.gen_label.new_label();

            // Lower condition
            let (if_assembly, if_symbol) = lower_exp(*if_exp, lowering_global);
            if_then_assembly.append(&mut if_assembly);

            // Create zero for use in comparison
            let zero_symbol = lowering_global.gen_sym.new_symbol();
            let zero_instruction = LIRInstruction::IntLit {
                assign_to: zero_symbol,
                value: 0,
            };
            let zero_assembly = LIRAssembly::Instruction(zero_instruction);
            if_then_assembly.push(zero_assembly);

            // Create zero comparison
            let zero_comparison = Comparison {
                c: ComparisonType::NotEqual,
                left: if_symbol,
                right: zero_symbol,
            };

            // Jump false conditional
            let jump_end_instruction = LIRInstruction::JumpC {
                to: end_label,
                condition: zero_comparison,
            };
            let jump_end_assembly = LIRAssembly::Instruction(jump_end_instruction);
            if_then_assembly.push(jump_end_assembly);

            // Lower then branch
            let (then_assembly, then_symbol) = lower_exp(*then_exp, lowering_global);
            if_then_assembly.append(&mut then_assembly);

            // Emit end label
            let end_label_assembly = LIRAssembly::Label(end_label);
            if_then_assembly.push(end_label_assembly);

            // TODO unsure what to return here as a symbol
            (if_then_assembly, then_symbol)
        }
        CheckedExp::While { while_exp, do_exp } => {
            let while_assembly = vec![];

            // Generate do, condition labels
            let do_label = lowering_global.gen_label.new_label();
            let condition_label = lowering_global.gen_label.new_label();

            // Jump condition
            let jump_condition_instruction = LIRInstruction::Jump {
                to: condition_label,
            };
            let jump_condition_assembly = LIRAssembly::Instruction(jump_condition_instruction);
            while_assembly.push(jump_condition_assembly);

            // Emit do label
            let do_label_assembly = LIRAssembly::Label(do_label);
            while_assembly.push(do_label_assembly);

            // Lower do_exp
            let (do_assembly, do_symbol) = lower_exp(*do_exp, lowering_global);
            while_assembly.append(&mut do_assembly);

            // Emit condition label
            let condition_label_assembly = LIRAssembly::Label(condition_label);
            while_assembly.push(condition_label_assembly);

            // Lower while_exp
            let (condition_assembly, condition_symbol) = lower_exp(*while_exp, lowering_global);
            while_assembly.append(&mut condition_assembly);

            // Create zero for use in comparison
            let zero_symbol = lowering_global.gen_sym.new_symbol();
            let zero_instruction = LIRInstruction::IntLit {
                assign_to: zero_symbol,
                value: 0,
            };
            let zero_assembly = LIRAssembly::Instruction(zero_instruction);
            while_assembly.push(zero_assembly);

            // Create not zero comparison
            let not_zero_comparison = Comparison {
                c: ComparisonType::NotEqual,
                left: condition_symbol,
                right: zero_symbol,
            };

            // Jump to do label if condition != 0
            let jump_do_instruction = LIRInstruction::JumpC {
                to: do_label,
                condition: not_zero_comparison,
            };
            let jump_do_assembly = LIRAssembly::Instruction(jump_do_instruction);
            while_assembly.push(jump_do_assembly);

            (while_assembly, do_symbol)
        }
        CheckedExp::For {
            id,
            for_exp,
            to_exp,
            do_exp,
        } => {
            let for_assembly = vec![];

            // Create one for use incrementing
            let one_symbol = lowering_global.gen_sym.new_symbol();
            let one_instruction = LIRInstruction::IntLit {
                assign_to: one_symbol,
                value: 1,
            };
            let one_assembly = LIRAssembly::Instruction(one_instruction);
            for_assembly.push(one_assembly);

            // Lower for_exp, for_symbol is current i

            // Label L1

            // Lower to_exp,

            // JumpC L2, for_symbol == to_symbol
            // Jump to end label if for_symbol == to_symbol

            // Lower do_exp

            // Increment for_symbol

            // Jump L1

            // Label L2
            // Emit end label

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

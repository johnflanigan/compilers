use crate::checked_grammar::*;
use crate::common::*;
use crate::lir::*;
use std::collections::HashMap;
use std::convert::TryInto;

#[derive(Debug)]
pub struct LoweringGlobal {
    pub symbol_table: HashMap<Symbol, TypeId>,
    pub types: HashMap<TypeId, Type>,
    pub gen_sym: SymbolGenerator,
    pub gen_label: LabelGenerator,
}

#[allow(dead_code, unused_variables)]
pub fn lower(
    type_checked_program: CheckedProgram,
) -> (LIRProgram, LabelGenerator, SymbolGenerator) {
    let mut lowering_global = LoweringGlobal {
        symbol_table: type_checked_program.symbol_table,
        types: type_checked_program.types,
        gen_sym: type_checked_program.gen_sym,
        gen_label: type_checked_program.gen_label,
    };

    /*
    pub struct CheckedProgram {
        pub function_symbols: HashMap<Label, FunctionType>,
        pub symbol_table: HashMap<Symbol, TypeId>,
        pub types: HashMap<TypeId, Type>,
        pub gen_sym: SymbolGenerator,
        pub gen_label: LabelGenerator,
        pub dec_list: VecDeque<CheckedTopLevelDec>,
    }
    */

    // Create LIRProgram fields
    // TODO fix this dummy function
    let mut main_function = LIRFunction {
        locals: vec![],
        arguments: vec![],
        return_symbol: lowering_global.gen_sym.new_symbol(),
        instruction_listing: vec![],
    };
    let mut other_functions = HashMap::new();

    for checked_top_level_dec in type_checked_program.dec_list {
        match checked_top_level_dec {
            CheckedTopLevelDec::FunDec { name, args, body } => {
                // Lower expression
                let (body_assembly, return_symbol) = lower_exp(*body, &mut lowering_global, None);

                // Create LIR function
                let lir_function = LIRFunction {
                    locals: vec![],
                    arguments: args.try_into().unwrap(),
                    return_symbol: return_symbol,
                    instruction_listing: body_assembly,
                };

                // Update LIR program
                if name == Label::Main {
                    main_function = lir_function;
                } else {
                    other_functions.insert(name, lir_function);
                }
            }
        }
    }

    let lir_program = LIRProgram {
        main_function: main_function,
        other_functions: other_functions,
    };

    (
        lir_program,
        lowering_global.gen_label,
        lowering_global.gen_sym,
    )
}

// Returns a sequence of LIR instructions and the symbol that will hold the result of those computations
fn lower_exp(
    checked_exp: CheckedExp,
    lowering_global: &mut LoweringGlobal,
    exit_label: Option<Label>,
) -> (Vec<LIRAssembly>, Symbol) {
    match checked_exp {
        CheckedExp::Break => {
            // Jump to exit
            let jump_instruction = LIRInstruction::Jump {
                to: exit_label.unwrap(),
            };
            let jump_assembly = LIRAssembly::Instruction(jump_instruction);

            // Generate symbol to return
            // TODO better if we make the return symbol an Option
            let break_symbol = lowering_global.gen_sym.new_symbol();

            (vec![jump_assembly], break_symbol)
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
        CheckedExp::LValue { lvalue } => {
            let (_, assembly, symbol) = lower_lvalue_value(lvalue, lowering_global, None);
            (assembly, symbol)
        }
        CheckedExp::Sequence { sequence } => {
            let mut sequence_assembly = vec![];
            let mut sequence_symbols = vec![];

            for exp in sequence {
                let (mut exp_assembly, exp_symbol) = lower_exp(exp, lowering_global, None);
                sequence_assembly.append(&mut exp_assembly);
                sequence_symbols.push(exp_symbol);
                // result_symbol = exp_symbol;
            }

            let final_symbol = sequence_symbols
                .pop()
                .unwrap_or(lowering_global.gen_sym.new_symbol());
            (sequence_assembly, final_symbol)
        }
        CheckedExp::Negate { exp } => {
            let (mut exp_assembly, exp_symbol) = lower_exp(*exp, lowering_global, None);

            // Create temporary symbol
            let negate_symbol = lowering_global.gen_sym.new_symbol();

            // Negate returned symbol
            let lir_negate_instruction = LIRInstruction::Negate {
                assign_to: negate_symbol,
                value: exp_symbol,
            };
            let lir_negate_assembly = LIRAssembly::Instruction(lir_negate_instruction);

            let mut negate_assembly = vec![];
            negate_assembly.append(&mut exp_assembly);
            negate_assembly.push(lir_negate_assembly);

            (negate_assembly, negate_symbol)
        }
        CheckedExp::Infix { left, op, right } => {
            // Call lower_exp on the left-hand operand to get left_assembly and left_symbol
            let (mut left_assembly, left_symbol) = lower_exp(*left, lowering_global, None);
            // Call lower_exp on the right-hand operand to get right_assembly and right_symbol
            let (mut right_assembly, right_symbol) = lower_exp(*right, lowering_global, None);

            // Generate a new temp symbol3
            let infix_symbol = lowering_global.gen_sym.new_symbol();

            let mut infix_assembly = vec![];
            infix_assembly.append(&mut left_assembly);
            infix_assembly.append(&mut right_assembly);

            // Try converting InfixSourceOp into InfixOp
            match op.try_into() {
                Ok(infix_op) => {
                    // Concatenate sequence1 + sequence2 + BinaryOp(symbol3, symbol1, Add, symbol2)
                    let binary_op_instruction = LIRInstruction::BinaryOp {
                        assign_to: infix_symbol,
                        left: left_symbol,
                        op: infix_op,
                        right: right_symbol,
                    };

                    let binary_op_assembly = LIRAssembly::Instruction(binary_op_instruction);

                    infix_assembly.push(binary_op_assembly);
                }
                Err(_) => {
                    // Generate true, false, and end labels
                    let true_label = lowering_global.gen_label.new_label();
                    let false_label = lowering_global.gen_label.new_label();
                    let end_label = lowering_global.gen_label.new_label();

                    // Jump to true of comparison is true
                    let jump_true_instruction = LIRInstruction::JumpC {
                        to: true_label,
                        condition: Comparison {
                            c: op.try_into().unwrap(),
                            left: left_symbol,
                            right: right_symbol,
                        },
                    };
                    let jump_true_assembly = LIRAssembly::Instruction(jump_true_instruction);
                    infix_assembly.push(jump_true_assembly);

                    // Else, jump to false
                    let jump_false_instruction = LIRInstruction::Jump { to: false_label };
                    let jump_false_assembly = LIRAssembly::Instruction(jump_false_instruction);
                    infix_assembly.push(jump_false_assembly);

                    // Emit true label
                    let true_label_assembly = LIRAssembly::Label(true_label);
                    infix_assembly.push(true_label_assembly);

                    // Assign 1 to infix_symbol
                    let one_instruction = LIRInstruction::IntLit {
                        assign_to: infix_symbol,
                        value: 1,
                    };
                    let one_assembly = LIRAssembly::Instruction(one_instruction);
                    infix_assembly.push(one_assembly);

                    // Jump end label
                    let jump_end_instruction = LIRInstruction::Jump { to: end_label };
                    let jump_end_assembly = LIRAssembly::Instruction(jump_end_instruction);
                    infix_assembly.push(jump_end_assembly);

                    // Emit false label
                    let false_label_assembly = LIRAssembly::Label(false_label);
                    infix_assembly.push(false_label_assembly);

                    // Assign 0 to infix_symbol
                    let zero_instruction = LIRInstruction::IntLit {
                        assign_to: infix_symbol,
                        value: 0,
                    };
                    let zero_assembly = LIRAssembly::Instruction(zero_instruction);
                    infix_assembly.push(zero_assembly);

                    // Emit end label
                    let end_label_assembly = LIRAssembly::Label(end_label);
                    infix_assembly.push(end_label_assembly);
                }
            }

            (infix_assembly, infix_symbol)
        }
        CheckedExp::ArrayCreate {
            length,
            initial_value,
        } => {
            // Create new symbol
            let array_symbol = lowering_global.gen_sym.new_symbol();

            // Load length and initial_value expressions into temporary symbols
            let (mut length_assembly, length_symbol) = lower_exp(*length, lowering_global, None);
            let (mut initial_value_assembly, initial_value_symbol) =
                lower_exp(*initial_value, lowering_global, None);

            // Call allocate_and_memset and set result to new symbol
            let call_instruction = LIRInstruction::Call {
                assign_to: array_symbol,
                function_name: Label::AllocateAndMemset,
                args: vec![length_symbol, initial_value_symbol],
            };
            let call_assembly = LIRAssembly::Instruction(call_instruction);

            // Append call instruction
            let mut array_assembly = vec![];
            array_assembly.append(&mut length_assembly);
            array_assembly.append(&mut initial_value_assembly);
            array_assembly.push(call_assembly);

            // Return the result
            (array_assembly, array_symbol)
        }
        CheckedExp::RecordCreate { fields } => {
            // Create new symbol for record
            let record_symbol = lowering_global.gen_sym.new_symbol();

            let mut record_assembly = vec![];

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
                let (mut exp_assembly, exp_symbol) = lower_exp(exp.clone(), lowering_global, None);
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
            let (_, mut left_assembly, left_symbol) =
                lower_lvalue_value(left, lowering_global, None);

            // Call lower_exp on the right-hand operand to get right_assembly and right_symbol
            let (mut right_assembly, right_symbol) = lower_exp(*right, lowering_global, None);

            let lir_assign_instruction = LIRInstruction::Assign {
                assign_to: left_symbol,
                id: right_symbol,
            };
            let lir_assign_assembly = LIRAssembly::Instruction(lir_assign_instruction);

            let mut assign_assembly = vec![];
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
            let mut if_then_else_assembly = vec![];

            // Generate true, false, and end labels
            let true_label = lowering_global.gen_label.new_label();
            let false_label = lowering_global.gen_label.new_label();
            let end_label = lowering_global.gen_label.new_label();

            // Lower condition
            let (mut if_assembly, if_symbol) = lower_exp(*if_exp, lowering_global, None);
            if_then_else_assembly.append(&mut if_assembly);

            // Generate new symbol to store result
            let if_then_else_symbol = lowering_global.gen_sym.new_symbol();

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

            // Jump false
            let jump_false_instruction = LIRInstruction::Jump { to: false_label };
            let jump_false_assembly = LIRAssembly::Instruction(jump_false_instruction);
            if_then_else_assembly.push(jump_false_assembly);

            // Emit true label
            let true_label_assembly = LIRAssembly::Label(true_label);
            if_then_else_assembly.push(true_label_assembly);

            // Lower then branch
            let (mut then_assembly, then_symbol) = lower_exp(*then_exp, lowering_global, None);
            if_then_else_assembly.append(&mut then_assembly);

            // Assign then_symbol to if_then_else_symbol
            let assign_true_symbol_instruction = LIRInstruction::Assign {
                assign_to: if_then_else_symbol,
                id: then_symbol,
            };
            let assign_true_symbol_assembly =
                LIRAssembly::Instruction(assign_true_symbol_instruction);
            if_then_else_assembly.push(assign_true_symbol_assembly);

            // Jump end label
            let jump_instruction = LIRInstruction::Jump { to: end_label };
            let jump_assembly = LIRAssembly::Instruction(jump_instruction);
            if_then_else_assembly.push(jump_assembly);

            // Emit false label
            let false_label_assembly = LIRAssembly::Label(false_label);
            if_then_else_assembly.push(false_label_assembly);

            // Lower else branch
            let (mut else_assembly, else_symbol) = lower_exp(*else_exp, lowering_global, None);
            if_then_else_assembly.append(&mut else_assembly);

            // Assign else_symbol to if_then_else_symbol
            let assign_else_symbol_instruction = LIRInstruction::Assign {
                assign_to: if_then_else_symbol,
                id: else_symbol,
            };
            let assign_else_symbol_assembly =
                LIRAssembly::Instruction(assign_else_symbol_instruction);
            if_then_else_assembly.push(assign_else_symbol_assembly);

            // Emit end label
            let end_label_assembly = LIRAssembly::Label(end_label);
            if_then_else_assembly.push(end_label_assembly);

            (if_then_else_assembly, if_then_else_symbol)
        }
        CheckedExp::IfThen { if_exp, then_exp } => {
            let mut if_then_assembly = vec![];

            // Generate end label
            let end_label = lowering_global.gen_label.new_label();

            // Lower condition
            let (mut if_assembly, if_symbol) = lower_exp(*if_exp, lowering_global, None);
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
            let (mut then_assembly, then_symbol) = lower_exp(*then_exp, lowering_global, None);
            if_then_assembly.append(&mut then_assembly);

            // Emit end label
            let end_label_assembly = LIRAssembly::Label(end_label);
            if_then_assembly.push(end_label_assembly);

            // TODO unsure what to return here as a symbol
            (if_then_assembly, then_symbol)
        }
        CheckedExp::While { while_exp, do_exp } => {
            let mut while_assembly = vec![];

            // Generate do, condition, end labels
            let do_label = lowering_global.gen_label.new_label();
            let condition_label = lowering_global.gen_label.new_label();
            let end_label = lowering_global.gen_label.new_label();

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
            let (mut do_assembly, do_symbol) = lower_exp(*do_exp, lowering_global, Some(end_label));
            while_assembly.append(&mut do_assembly);

            // Emit condition label
            let condition_label_assembly = LIRAssembly::Label(condition_label);
            while_assembly.push(condition_label_assembly);

            // Lower while_exp
            let (mut condition_assembly, condition_symbol) =
                lower_exp(*while_exp, lowering_global, Some(end_label));
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

            // Emit end label (for use in break)
            let end_label_assembly = LIRAssembly::Label(end_label);
            while_assembly.push(end_label_assembly);

            (while_assembly, do_symbol)
        }
        CheckedExp::For {
            id,
            for_exp,
            to_exp,
            do_exp,
        } => {
            let mut for_loop_assembly = vec![];

            // Generate for loop, end labels
            let for_loop_label = lowering_global.gen_label.new_label();
            let end_label = lowering_global.gen_label.new_label();

            // Create one for use incrementing
            let one_symbol = lowering_global.gen_sym.new_symbol();
            let one_instruction = LIRInstruction::IntLit {
                assign_to: one_symbol,
                value: 1,
            };
            let one_assembly = LIRAssembly::Instruction(one_instruction);
            for_loop_assembly.push(one_assembly);

            // Lower for_exp, for_symbol is current i
            let (mut for_assembly, for_symbol) =
                lower_exp(*for_exp, lowering_global, Some(end_label));
            for_loop_assembly.append(&mut for_assembly);

            // Assign for_exp to id
            let id_assign_instruction = LIRInstruction::Assign {
                assign_to: id,
                id: for_symbol,
            };
            let id_assign_assembly = LIRAssembly::Instruction(id_assign_instruction);
            for_loop_assembly.push(id_assign_assembly);

            // Lower to_exp
            let (mut to_assembly, to_symbol) = lower_exp(*to_exp, lowering_global, Some(end_label));
            for_loop_assembly.append(&mut to_assembly);

            // Emit for loop label
            let for_loop_label_assembly = LIRAssembly::Label(for_loop_label);
            for_loop_assembly.push(for_loop_label_assembly);

            // Compare if id > to_symbol
            let greater_than_comparison = Comparison {
                c: ComparisonType::GreaterThan,
                left: id,
                right: to_symbol,
            };

            // Jump to end label if id > to_symbol met
            let jump_end_instruction = LIRInstruction::JumpC {
                to: end_label,
                condition: greater_than_comparison,
            };
            let jump_end_assembly = LIRAssembly::Instruction(jump_end_instruction);
            for_loop_assembly.push(jump_end_assembly);

            // Lower do_exp
            let (mut do_assembly, do_symbol) = lower_exp(*do_exp, lowering_global, Some(end_label));
            for_loop_assembly.append(&mut do_assembly);

            // Increment id
            let increment_instruction = LIRInstruction::BinaryOp {
                assign_to: id,
                left: id,
                op: InfixOp::Add,
                right: one_symbol,
            };
            let increment_assembly = LIRAssembly::Instruction(increment_instruction);
            for_loop_assembly.push(increment_assembly);

            // Jump to top of for loop
            let jump_for_loop_instruction = LIRInstruction::Jump { to: for_loop_label };
            let jump_for_loop_assembly = LIRAssembly::Instruction(jump_for_loop_instruction);
            for_loop_assembly.push(jump_for_loop_assembly);

            // Emit end label
            let end_label_assembly = LIRAssembly::Label(end_label);
            for_loop_assembly.push(end_label_assembly);

            (for_loop_assembly, do_symbol)
        }
        CheckedExp::Let { let_exp, in_exp } => {
            let mut let_assembly = vec![];

            for dec in let_exp {
                match dec {
                    CheckedDec::VarDec { name, value } => {
                        let (mut value_assembly, value_symbol) =
                            lower_exp(value, lowering_global, None);
                        let_assembly.append(&mut value_assembly);

                        let assign_instruction = LIRInstruction::Assign {
                            assign_to: name,
                            id: value_symbol,
                        };
                        let assign_assembly = LIRAssembly::Instruction(assign_instruction);
                        let_assembly.push(assign_assembly);
                    }
                }
            }

            let (mut in_assembly, in_symbol) = lower_exp(*in_exp, lowering_global, None);
            let_assembly.append(&mut in_assembly);

            (let_assembly, in_symbol)
        }
        CheckedExp::Call {
            function_name,
            args,
        } => {
            let mut call_assembly = vec![];
            let mut arg_symbols = vec![];

            for arg in args {
                let (mut arg_assembly, arg_symbol) = lower_exp(arg, lowering_global, None);

                call_assembly.append(&mut arg_assembly);
                arg_symbols.push(arg_symbol);
            }

            let result_symbol = lowering_global.gen_sym.new_symbol();

            let lir_call_instruction = LIRInstruction::Call {
                assign_to: result_symbol,
                function_name: function_name,
                args: arg_symbols,
            };
            let lir_call_assembly = LIRAssembly::Instruction(lir_call_instruction);
            call_assembly.push(lir_call_assembly);

            (call_assembly, result_symbol)
        }
    }
}

fn lower_lvalue_value(
    checked_lvalue: CheckedLValue,
    lowering_global: &mut LoweringGlobal,
    exit_label: Option<Label>,
) -> (Type, Vec<LIRAssembly>, Symbol) {
    match checked_lvalue {
        CheckedLValue::Id { name } => {
            // Create temporary symbol
            let symbol = lowering_global.gen_sym.new_symbol();

            // Assign Id to temporary symbol
            let instruction = LIRInstruction::Assign {
                assign_to: symbol,
                id: name,
            };
            let assembly = LIRAssembly::Instruction(instruction);

            // Lookup type
            let type_id = lowering_global.symbol_table.get(&name).unwrap();
            let type_value = lowering_global.types.get(&type_id).unwrap();

            // Return assembly instruction and temporary symbol
            (type_value.clone(), vec![assembly], symbol)
        }
        CheckedLValue::Subscript { array, index } => {
            let mut subscript_assembly = vec![];

            // Create temporary symbol
            let symbol = lowering_global.gen_sym.new_symbol();

            // Get assembly and symbol for checked
            let (type_value, mut array_assembly, array_symbol) =
                lower_lvalue_value(*array, lowering_global, None);
            subscript_assembly.append(&mut array_assembly);

            // Geneate index symbol
            let (mut index_assembly, index_symbol) = lower_exp(*index, lowering_global, None);
            subscript_assembly.append(&mut index_assembly);

            // Load from memory offset
            let load_from_memory_at_offset_instruction = LIRInstruction::LoadFromMemoryAtOffset {
                assign_to: symbol,
                location: array_symbol,
                offset: index_symbol,
            };
            let load_from_memory_at_offset_assembly =
                LIRAssembly::Instruction(load_from_memory_at_offset_instruction);

            subscript_assembly.push(load_from_memory_at_offset_assembly);

            // Return assembly instruction and temporary symbol
            (type_value, subscript_assembly, symbol)
        }
        CheckedLValue::FieldExp { record, field } => {
            let mut field_assembly = vec![];

            // Create temporary symbol
            let symbol = lowering_global.gen_sym.new_symbol();

            // Get assembly and symbol for checked
            let (type_value, mut record_assembly, record_symbol) =
                lower_lvalue_value(*record, lowering_global, None);
            field_assembly.append(&mut record_assembly);

            match type_value {
                Type::Record(v) => {
                    let index = v.iter().position(|r| r.0 == field).unwrap();
                    let element_type = lowering_global.types.get(&v.get(index).unwrap().1).unwrap();

                    // Add 1 because 0 represents record address
                    let field_index = (index as i64) + 1;

                    // Store index in symbol
                    let index_symbol = lowering_global.gen_sym.new_symbol();
                    let index_instruction = LIRInstruction::IntLit {
                        assign_to: index_symbol,
                        value: field_index,
                    };
                    let index_assembly = LIRAssembly::Instruction(index_instruction);
                    field_assembly.push(index_assembly);

                    // Load from memory offset
                    let load_from_memory_at_offset_instruction =
                        LIRInstruction::LoadFromMemoryAtOffset {
                            assign_to: symbol,
                            location: record_symbol,
                            offset: index_symbol,
                        };
                    let load_from_memory_at_offset_assembly =
                        LIRAssembly::Instruction(load_from_memory_at_offset_instruction);
                    field_assembly.push(load_from_memory_at_offset_assembly);

                    // Return field assembly and symbol
                    (element_type.clone(), field_assembly, symbol)
                }
                _ => {
                    panic!("Unexpected type value");
                }
            }
        }
    }
}

// fn lower_lvalue_assigning_symbol(
//     checked: CheckedLValue,
//     symbol: Symbol,
//     break_to: Option<Label>,
//     gen: &mut LoweringGlobal,
// ) -> Vec<LIRAssembly> {
//     unimplemented!();
// }

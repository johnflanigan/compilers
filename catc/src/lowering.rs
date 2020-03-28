use crate::checked_grammar::*;
use crate::common::*;
use crate::lir::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct LoweringGlobal {
    pub main_function: LIRFunction,
    pub other_functions: HashMap<Label, LIRFunction>,
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct LIRProgram {
//     pub main_function: LIRFunction,
//     pub other_functions: HashMap<Label, LIRFunction>,
// }

// #[derive(Debug, Clone, Deserialize, Serialize)]
// pub struct CheckedProgram {
//     pub function_symbols: HashMap<Label, FunctionType>,
//     pub symbol_table: HashMap<Symbol, TypeId>,
//     pub types: HashMap<TypeId, Type>,
//     pub gen_sym: SymbolGenerator,
//     pub gen_label: LabelGenerator,
//     pub dec_list: VecDeque<CheckedTopLevelDec>,
// }

#[allow(dead_code, unused_variables)]
pub fn lower(type_checked_program: CheckedProgram) -> LIRProgram {
    for checked_top_level_dec in type_checked_program.dec_list {
        match checked_top_level_dec {
            CheckedTopLevelDec::FunDec { name, args, body } => match *body {
                CheckedExp::Break => {}
                CheckedExp::IntLit { value } => {}
                CheckedExp::StringLit { value } => {}
                CheckedExp::LValue { lvalue } => {}
                CheckedExp::Sequence { sequence } => {}
                CheckedExp::Negate { exp } => {}
                CheckedExp::Infix { left, op, right } => {}
                CheckedExp::ArrayCreate {
                    length,
                    initial_value,
                } => {}
                CheckedExp::RecordCreate { fields } => {}
                CheckedExp::Assign { left, right } => {}
                CheckedExp::IfThenElse {
                    if_exp,
                    then_exp,
                    else_exp,
                } => {}
                CheckedExp::IfThen { if_exp, then_exp } => {}
                CheckedExp::While { while_exp, do_exp } => {}
                CheckedExp::For {
                    id,
                    for_exp,
                    to_exp,
                    do_exp,
                } => {}
                CheckedExp::Let { let_exp, in_exp } => {}
                CheckedExp::Call {
                    function_name,
                    args,
                } => {}
            },
        }
    }
    unimplemented!();
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

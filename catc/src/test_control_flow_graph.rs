use std::collections::HashMap;

use crate::common::{Comparison, ComparisonType};
use crate::common::{LabelGenerator, SymbolGenerator};

use crate::lir::LIRInstruction::*;
use crate::lir::*;

use crate::control_flow_graph::{
    construct_control_flow_graph_lir, construct_control_flow_graph_x64s, liveness,
};

// Less simple examples use the other parts of your compiler to make the test
// easier to understand. You may just run the simpler tests by commenting out
// these lines and running cargo test. For example if you want to run the
// lir_cfg_test_jumpc_label test:
// cargo test test_control_flow_graph::lir_cfg_test_jumpc_label
//
// For more diagnostic information (like back traces and the information which)
// the test prints out run:
// RUST_BACKTRACE=1 cargo test test_control_flow_graph::lir_cfg_test_jumpc_label -- --nocapture
use crate::backend::{fix_up, select, GlobalInfo};
use crate::check_type::type_check;
use crate::lowering::lower;
use crate::parser::ProgramParser;

#[test]
fn lir_cfg_test_simple() {
    let mut sg = SymbolGenerator::new();
    let s = sg.new_symbol();
    let s1 = sg.new_symbol();

    let f = LIRFunction {
        locals: vec![],
        arguments: vec![],
        return_symbol: s,
        instruction_listing: vec![
            linst!(IntLit {
                assign_to: s,
                value: 10
            }),
            linst!(Assign {
                assign_to: s1,
                id: s
            }),
        ],
    };
    let label_gen = LabelGenerator::new();
    let symbol_gen = SymbolGenerator::new();
    let mut state = GlobalInfo {
        label_gen,
        symbol_gen,
        string_literals: HashMap::new(),
    };
    let selected_program = select(
        LIRProgram {
            main_function: f,
            other_functions: HashMap::new(),
        },
        &mut state,
    );
    let single_memory_op = fix_up(selected_program);

    let inital_cfg = construct_control_flow_graph_x64s(&single_memory_op.main_function);
    println!("CFG:------\n{}", inital_cfg.to_dot());

    let cfg = liveness(inital_cfg);
    println!("{:?}", cfg);
    println!("--------------------------\n{}", cfg.to_dot());
}

#[test]
fn lir_cfg_test_jump() {
    let mut sg = SymbolGenerator::new();
    let s = sg.new_symbol();

    let mut lg = LabelGenerator::new();
    let l = lg.new_label();

    let f = LIRFunction {
        locals: vec![],
        arguments: vec![],
        return_symbol: s,
        instruction_listing: vec![
            linst!(Jump { to: l }),
            linst!(IntLit {
                assign_to: s,
                value: 10
            }),
            LIRAssembly::Label(l),
        ],
    };

    let cfg = liveness(construct_control_flow_graph_lir(&f));
    println!("{:?}", cfg);
    println!("--------------------------\n{}", cfg.to_dot());
}

#[test]
fn lir_cfg_test_jumpc() {
    let mut sg = SymbolGenerator::new();
    let s = sg.new_symbol();

    let mut lg = LabelGenerator::new();
    let l = lg.new_label();

    let f = LIRFunction {
        locals: vec![],
        arguments: vec![],
        return_symbol: s,
        instruction_listing: vec![
            linst!(JumpC {
                to: l,
                condition: Comparison {
                    c: ComparisonType::Equal,
                    left: s,
                    right: s
                }
            }),
            linst!(IntLit {
                assign_to: s,
                value: 10
            }),
            LIRAssembly::Label(l),
            linst!(IntLit {
                assign_to: s,
                value: 3
            }),
        ],
    };

    let cfg = liveness(construct_control_flow_graph_lir(&f));
    println!("{:?}", cfg);
    println!("--------------------------\n{}", cfg.to_dot());
}

#[test]
fn lir_cfg_test_jumpc_label() {
    let mut sg = SymbolGenerator::new();
    let s = sg.new_symbol();

    let mut lg = LabelGenerator::new();
    let l = lg.new_label();

    let f = LIRFunction {
        locals: vec![],
        arguments: vec![],
        return_symbol: s,
        instruction_listing: vec![
            linst!(IntLit {
                assign_to: s,
                value: 10
            }),
            linst!(JumpC {
                to: l,
                condition: Comparison {
                    c: ComparisonType::Equal,
                    left: s,
                    right: s
                }
            }),
            linst!(IntLit {
                assign_to: s,
                value: 10
            }),
            LIRAssembly::Label(l),
        ],
    };

    let cfg = liveness(construct_control_flow_graph_lir(&f));
    println!("{:?}", cfg);
    println!("--------------------------\n{}", cfg.to_dot());

    let label_gen = LabelGenerator::new();
    let symbol_gen = SymbolGenerator::new();

    let mut state = GlobalInfo {
        label_gen,
        symbol_gen,
        string_literals: HashMap::new(),
    };
    let selected_program = select(
        LIRProgram {
            main_function: f,
            other_functions: HashMap::new(),
        },
        &mut state,
    );
    let single_memory_op = fix_up(selected_program);

    let inital_cfg = construct_control_flow_graph_x64s(&single_memory_op.main_function);
    println!("CFG:------\n{}", inital_cfg.to_dot());

    let cfg = liveness(inital_cfg);
    println!("{:?}", cfg);
    println!("--------------------------\n{}", cfg.to_dot());
}

#[test]
fn test_fib() {
    let program = r#"
    function fib (n : int) -> int {
        if n = 0 or n = 1 then
            1
        else
            fib(n - 1) + fib(n - 2)
    }

    function main () -> void {
        let var res : int := fib(50)
        in print_line_int(res)
        end
    }
    "#;
    let checked_program = type_check(ProgramParser::new().parse(program).unwrap()).unwrap();
    let (lir_program, _, _) = lower(checked_program);

    let cfg = liveness(construct_control_flow_graph_lir(&lir_program.main_function));
    let cfgs: HashMap<_, _> = lir_program
        .other_functions
        .iter()
        .map(|(l, f)| (l, liveness(construct_control_flow_graph_lir(&f))))
        .collect();
    println!("main:--------------------------\n{}", cfg.to_dot());
    for (l, cfg) in cfgs.iter() {
        println!("{}:--------------------------\n{}", l, cfg.to_dot());
    }
}

#[test]
fn test_if() {
    let program = r#"
    function main () -> int {
        let var i:int := (if 1 then
            1
        else
            0) in i end
    }
    "#;
    let checked_program = type_check(ProgramParser::new().parse(program).unwrap()).unwrap();
    let (lir_program, _, _) = lower(checked_program);
    println!("{:?}", lir_program);

    let cfg = liveness(construct_control_flow_graph_lir(&lir_program.main_function));
    println!("main:--------------------------\n{}", cfg.to_dot());

    let label_gen = LabelGenerator::new();
    let symbol_gen = SymbolGenerator::new();
    let mut state = GlobalInfo {
        label_gen,
        symbol_gen,
        string_literals: HashMap::new(),
    };
    let selected_program = select(lir_program, &mut state);
    let single_memory_op = fix_up(selected_program);

    let inital_cfg = construct_control_flow_graph_x64s(&single_memory_op.main_function);
    println!("CFG:------\n{}", inital_cfg.to_dot());

    let cfg = liveness(inital_cfg);
    println!("{:?}", cfg);
    println!("--------------------------\n{}", cfg.to_dot());
}

#[test]
fn test_for() {
    let program = r#"
    function main () -> void {
        for i := 0 to 10 do ()
    }
    "#;
    let checked_program = type_check(ProgramParser::new().parse(program).unwrap()).unwrap();
    let (lir_program, _, _) = lower(checked_program);
    println!("{:?}", lir_program);

    let cfg = liveness(construct_control_flow_graph_lir(&lir_program.main_function));
    println!("main:--------------------------\n{}", cfg.to_dot());

    let label_gen = LabelGenerator::new();
    let symbol_gen = SymbolGenerator::new();
    let mut state = GlobalInfo {
        label_gen,
        symbol_gen,
        string_literals: HashMap::new(),
    };
    let selected_program = select(lir_program, &mut state);
    let single_memory_op = fix_up(selected_program);

    let inital_cfg = construct_control_flow_graph_x64s(&single_memory_op.main_function);
    println!("CFG:------\n{}", inital_cfg.to_dot());

    let cfg = liveness(inital_cfg);
    println!("{:?}", cfg);
    println!("--------------------------\n{}", cfg.to_dot());
}

#[test]
fn test_for_sum() {
    let program = r#"
    function main () -> void {
        let var sum: int := 0 in
            for i := 0 to 10 do (sum := sum + 1; ()) end
    }
    "#;
    let checked_program = type_check(ProgramParser::new().parse(program).unwrap()).unwrap();
    let (lir_program, _, _) = lower(checked_program);

    let cfg = liveness(construct_control_flow_graph_lir(&lir_program.main_function));
    let cfgs: HashMap<_, _> = lir_program
        .other_functions
        .iter()
        .map(|(l, f)| (l, liveness(construct_control_flow_graph_lir(&f))))
        .collect();
    println!("main:--------------------------\n{}", cfg.to_dot());
    for (l, cfg) in cfgs.iter() {
        println!("{}:--------------------------\n{}", l, cfg.to_dot());
    }
}

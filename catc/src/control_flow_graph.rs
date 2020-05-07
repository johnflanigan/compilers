use petgraph::graph::{DefaultIx, NodeIndex};
use petgraph::stable_graph::StableGraph;
use petgraph::Direction;

use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::{Debug, Display};

use crate::common::Comparison;
use crate::common::{Label, Symbol};

use crate::lir::LIRInstruction::*;
use crate::lir::*;

use crate::x64::{X64Value, X64opCode};

use crate::x64s::{SOperand, SOperands, X64SAssembly, X64SFunction, X64SInstruction};

/*
 * GenKill Trait
 *
 * This trait provides an interface to get the gen (a.k.a use) and kill
 * (a.k.a def) sets of a given statement. Those sets are defined as follows:
 *
 * gen(S) is the set of variables that are used in S before any assignment
 * kill(S) is the set of variables that are assigned a value in S
 *
 * S here is a statement
 */
pub trait GenKill {
    fn gen(&self) -> HashSet<Symbol>;
    fn kill(&self) -> HashSet<Symbol>;
}

/*
 * GenKill implementation for LIRAssembly
 *
 * For each LIRAssembly statement fill in the variables which are members of
 * the gen and kill sets for that statement (see the GenKill implementation
 * of X64SAssembly as an example)
 */
impl GenKill for LIRAssembly {
    fn gen(&self) -> HashSet<Symbol> {
        match self {
            LIRAssembly::Label(_) => vec![],
            LIRAssembly::Instruction(Nop) => vec![],
            LIRAssembly::Instruction(IntLit { assign_to, value }) => vec![],
            LIRAssembly::Instruction(StringLit { assign_to, value }) => vec![],
            LIRAssembly::Instruction(StoreToMemoryAtOffset {
                location,
                offset,
                value,
            }) => vec![value],
            LIRAssembly::Instruction(LoadFromMemoryAtOffset {
                assign_to,
                location,
                offset,
            }) => vec![location, offset],
            LIRAssembly::Instruction(Assign { assign_to, id }) => vec![id],
            LIRAssembly::Instruction(Negate { assign_to, value }) => vec![value],
            LIRAssembly::Instruction(BinaryOp {
                assign_to,
                left,
                op,
                right,
            }) => vec![left, right],
            LIRAssembly::Instruction(Call {
                assign_to,
                function_name,
                args,
            }) => args.iter().map(|arg| arg).collect(),
            LIRAssembly::Instruction(Jump { .. }) => vec![],
            LIRAssembly::Instruction(JumpC {
                condition: Comparison { left, right, .. },
                ..
            }) => vec![left, right],
        }
        .into_iter()
        .cloned()
        .collect()
    }

    fn kill(&self) -> HashSet<Symbol> {
        match self {
            LIRAssembly::Label(_) => vec![],
            LIRAssembly::Instruction(Nop) => vec![],
            LIRAssembly::Instruction(IntLit { assign_to, value }) => vec![assign_to],
            LIRAssembly::Instruction(StringLit { assign_to, value }) => vec![assign_to],
            LIRAssembly::Instruction(StoreToMemoryAtOffset {
                location,
                offset,
                value,
            }) => vec![location, offset],
            LIRAssembly::Instruction(LoadFromMemoryAtOffset {
                assign_to,
                location,
                offset,
            }) => vec![assign_to],
            LIRAssembly::Instruction(Assign { assign_to, id }) => vec![assign_to],
            LIRAssembly::Instruction(Negate { assign_to, value }) => vec![assign_to],
            LIRAssembly::Instruction(BinaryOp {
                assign_to,
                left,
                op,
                right,
            }) => vec![assign_to],
            LIRAssembly::Instruction(Call {
                assign_to,
                function_name,
                args,
            }) => vec![assign_to],
            LIRAssembly::Instruction(Jump { .. }) => vec![],
            LIRAssembly::Instruction(JumpC {
                condition: Comparison { left, right, .. },
                ..
            }) => vec![],
        }
        .into_iter()
        .cloned()
        .collect()
    }
}

/*
 * GenKill implementation for X64SAssembly
 */
impl GenKill for X64SAssembly {
    fn gen(&self) -> HashSet<Symbol> {
        match self {
            X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::Add,
                args,
            })
            | X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::Sub,
                args,
            })
            | X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::Or,
                args,
            })
            | X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::And,
                args,
            })
            | X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::Cmp,
                args,
            }) => match args {
                SOperands::Two(SOperand::Symbol(source), SOperand::Symbol(dest))
                | SOperands::Two(SOperand::MemorySym(source), SOperand::MemorySym(dest))
                | SOperands::Two(SOperand::MemorySym(source), SOperand::Symbol(dest))
                | SOperands::Two(SOperand::Symbol(source), SOperand::MemorySym(dest)) => {
                    vec![source, dest]
                }

                SOperands::Two(SOperand::MemorySym(source), _)
                | SOperands::Two(SOperand::Symbol(source), _) => vec![source],

                SOperands::Two(_, SOperand::MemorySym(dest))
                | SOperands::Two(_, SOperand::Symbol(dest)) => vec![dest],

                SOperands::Two(_, _) => vec![],
                _ => panic!("Unknown X64Assembly shape"),
            },

            X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::Lea,
                args,
            })
            | X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::Movq,
                args,
            }) => match args {
                SOperands::Two(SOperand::MemorySym(source), _)
                | SOperands::Two(SOperand::Symbol(source), _) => vec![source],

                SOperands::Two(_, _) => vec![],
                _ => panic!("Unknown X64Assembly shape"),
            },
            X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::Shl,
                args,
            }) => match args {
                SOperands::Two(_, SOperand::Symbol(dest))
                | SOperands::Two(_, SOperand::MemorySym(dest)) => vec![dest],

                SOperands::Two(_, _) => vec![],
                _ => panic!("Unknown X64Assembly shape"),
            },
            X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::IMulq,
                args,
            })
            | X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::IDivq,
                args,
            })
            | X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::Neg,
                args,
            })
            | X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::Push,
                args,
            }) => match args {
                SOperands::One(SOperand::Symbol(source))
                | SOperands::One(SOperand::MemorySym(source)) => vec![source],

                SOperands::One(_) => vec![],

                _ => panic!("Unknown X64SAssembly shape"),
            },

            X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::Pop,
                ..
            })
            | X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::Call,
                ..
            })
            | X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::Jmp,
                ..
            })
            | X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::Je,
                ..
            })
            | X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::Jne,
                ..
            })
            | X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::Jg,
                ..
            })
            | X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::Jge,
                ..
            })
            | X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::Jl,
                ..
            })
            | X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::Jle,
                ..
            })
            | X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::Ret,
                ..
            })
            | X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::Nop,
                ..
            })
            | X64SAssembly::Label(_) => vec![],
        }
        .into_iter()
        .copied()
        .collect()
    }

    fn kill(&self) -> HashSet<Symbol> {
        match self {
            X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::Add,
                args,
            })
            | X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::Sub,
                args,
            })
            | X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::Or,
                args,
            })
            | X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::And,
                args,
            })
            | X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::Cmp,
                args,
            })
            | X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::Shl,
                args,
            })
            | X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::Lea,
                args,
            })
            | X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::Movq,
                args,
            }) => match args {
                SOperands::Two(_, SOperand::Symbol(dest))
                | SOperands::Two(_, SOperand::MemorySym(dest)) => vec![dest],

                SOperands::Two(_, _) => vec![],
                _ => panic!("Unknown X64Assembly shape"),
            },

            X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::Neg,
                args,
            })
            | X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::Pop,
                args,
            }) => match args {
                SOperands::One(SOperand::Symbol(dest))
                | SOperands::One(SOperand::MemorySym(dest)) => vec![dest],

                SOperands::One(_) => vec![],

                _ => panic!("Unknown X64SAssembly shape"),
            },

            X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::Push,
                ..
            })
            | X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::Call,
                ..
            })
            | X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::IMulq,
                ..
            })
            | X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::IDivq,
                ..
            })
            | X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::Jmp,
                ..
            })
            | X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::Je,
                ..
            })
            | X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::Jne,
                ..
            })
            | X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::Jg,
                ..
            })
            | X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::Jge,
                ..
            })
            | X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::Jl,
                ..
            })
            | X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::Jle,
                ..
            })
            | X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::Ret,
                ..
            })
            | X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::Nop,
                ..
            })
            | X64SAssembly::Label(_) => vec![],
        }
        .into_iter()
        .copied()
        .collect()
    }
}

/*
 * Node is either the Entry (Start) or Exit (End) of the sequence of instruction
 * or Index which, indicates which statement the node is associated with
 */
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Node {
    Start,
    End,
    Index(usize),
}

/*
 * ControlFlowGraph stores a graph and related data.
 *
 * graph:
 *  a graph which is the cfg of the instructions, the weights of the nodes
 *  are a distinguished Start and End node as well as a node for each
 *  instruction indicated by index in instructions
 * instructions:
 *  a vector of statements
 * data:
 *  a set of symbols associated with each node for liveness analysis
 *  this is the part of the graph will be modified by liveness analysis
 *  The keys of the HashMap are Node's (Start, End and Index for each index in
 *  instructions).
 */
#[derive(Debug)]
pub struct ControlFlowGraph<INSTR: Display + Debug + GenKill> {
    pub graph: StableGraph<Node, ()>,
    pub instructions: Vec<INSTR>,
    pub data: HashMap<Node, HashSet<Symbol>>,
}

/*
 * Helper functions for ControlFlowGraph
 */
impl<INSTR: Display + Debug + GenKill> ControlFlowGraph<INSTR> {
    /*
     * Get a list of the indices of all nodes in the graph
     */
    pub fn node_indices(&self) -> VecDeque<NodeIndex> {
        self.graph.node_indices().rev().collect()
    }

    /*
     * Given a node index get the associated livein data (for liveness analysis)
     */
    pub fn node_data(&self, n: NodeIndex) -> HashSet<Symbol> {
        self.data.get(&self.node_weight(n)).unwrap().clone()
    }

    /*
     * Given a node index make the text with the livein set and statement
     */
    pub fn node_instruction_text(&self, n: NodeIndex) -> String {
        match self.node_weight(n) {
            Node::Index(i) => {
                let instruction_text = format!("{}", self.instructions[i])
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ");
                format!(
                    "{}\n{}",
                    display_liveset(&self.data[&Node::Index(i)]),
                    instruction_text
                )
            }
            Node::Start => format!("{}\nStart", display_liveset(&self.data[&Node::Start])),
            Node::End => format!("{}\nEnd", display_liveset(&self.data[&Node::End])),
        }
    }

    /*
     * Node weight get the node index's node weight (which may be an Index or
     * the Start or End).
     */
    pub fn node_weight(&self, n: NodeIndex) -> Node {
        *self.graph.node_weight(n).unwrap()
    }

    /*
     * Get the successors of a given NodeIndex.
     */
    pub fn succ(&self, n: NodeIndex) -> Vec<NodeIndex> {
        self.graph
            .neighbors_directed(n, Direction::Outgoing)
            .collect()
    }

    /*
     * Get the predecessors of a given NodeIndex.
     */
    pub fn pred(&self, n: NodeIndex) -> Vec<NodeIndex> {
        self.graph
            .neighbors_directed(n, Direction::Incoming)
            .collect()
    }

    /*
     * Get a given node index get the live in sets of the successor
     */
    pub fn succ_data(&self, n: NodeIndex) -> Vec<HashSet<Symbol>> {
        self.succ(n)
            .into_iter()
            .map(|neighbor| self.node_data(neighbor))
            .collect()
    }

    /*
     * Get a given node index get the live in sets of the predecessor
     */
    pub fn pred_data(&self, n: NodeIndex) -> Vec<HashSet<Symbol>> {
        self.pred(n)
            .into_iter()
            .map(|neighbor| self.node_data(neighbor))
            .collect()
    }

    /*
     * Get the gen set of the statement associated with the given node index
     */
    pub fn gen_node(&self, n: NodeIndex) -> HashSet<Symbol> {
        match self.graph.node_weight(n).unwrap() {
            Node::Index(i) => self.instructions.get(*i).unwrap().gen(),
            _ => HashSet::new(),
        }
    }

    /*
     * Get the kill set of the statement associated with the given node index
     */
    pub fn kill_node(&self, n: NodeIndex) -> HashSet<Symbol> {
        match self.graph.node_weight(n).unwrap() {
            Node::Index(i) => self.instructions.get(*i).unwrap().kill(),
            _ => HashSet::new(),
        }
    }

    /*
     * Convert the ControlFlowGraph to a dot format to visualize it
     * You can use the output of this function to visualize the CFG
     * online: https://edotor.net/
     */
    pub fn to_dot(&self) -> String {
        let digraph_start = String::from("digraph {");
        let labels = self
            .node_indices()
            .into_iter()
            .map(|ni| {
                format!(
                    "\t{:?} [label={:?}]",
                    ni.index(),
                    self.node_instruction_text(ni)
                )
            })
            .collect::<Vec<_>>()
            .join("\n");
        let edges = self
            .graph
            .edge_indices()
            .map(|ei| self.graph.edge_endpoints(ei).unwrap())
            .map(|(ns, ne)| format!("\t{:?} -> {:?}", ns.index(), ne.index()))
            .collect::<Vec<_>>()
            .join("\n");
        let digraph_end = String::from("}");
        vec![digraph_start, labels, edges, digraph_end].join("\n")
    }
}

/*
 * Convert a Liveset to a String
 */
fn display_liveset(liveset: &HashSet<Symbol>) -> String {
    let start = String::from("{");
    let elems = liveset
        .iter()
        .map(|s| format!("{}", s))
        .collect::<Vec<_>>()
        .join(", ");
    let end = String::from("}");
    vec![start, elems, end].join("")
}

/*
 * Union set1 and set2
 */
pub fn union<T: std::marker::Copy + std::hash::Hash + std::cmp::Eq>(
    set1: HashSet<T>,
    set2: HashSet<T>,
) -> HashSet<T> {
    set1.union(&set2).copied().collect()
}

/*
 * Union all sets in a vector
 */
pub fn union_all<T: std::marker::Copy + std::hash::Hash + std::cmp::Eq>(
    data: Vec<HashSet<T>>,
) -> HashSet<T> {
    data.into_iter()
        .fold(HashSet::new(), |acc, set| union(acc, set))
}

/*
 * Take the set difference: set1 - set2
 */
pub fn difference<T: std::marker::Copy + std::hash::Hash + std::cmp::Eq>(
    set1: HashSet<T>,
    set2: HashSet<T>,
) -> HashSet<T> {
    set1.difference(&set2).cloned().collect()
}

/*
 * Liveness
 *
 * Given a control flow graph with all empty live sets preform the data-flow
 * analysis to get the live variable information.
 */
pub fn liveness<ASSEM: GenKill + Debug + Display>(
    mut cfg: ControlFlowGraph<ASSEM>,
) -> ControlFlowGraph<ASSEM> {
    // make a work list of all nodes
    let mut work_list = cfg.node_indices();
    let mut live_in: HashMap<NodeIndex, HashSet<Symbol>> = HashMap::new();
    let mut live_out: HashMap<NodeIndex, HashSet<Symbol>> = HashMap::new();

    // while the work list is not empty:
    while !work_list.is_empty() {
        // remove node n from the work list
        let node = work_list.pop_front().unwrap();

        // live_out[n] = union in[s] for all s which are successors of n
        let successors = cfg.succ(node);
        let mut successors_live_in: Vec<HashSet<Symbol>> = vec![];
        for successor in successors {
            if live_in.contains_key(&successor) {
                successors_live_in.push(live_in.get(&successor).unwrap().clone());
            }
        }
        live_out.insert(node, union_all(successors_live_in));

        // old_live_in = in[n]
        // live_in[n] = gen[n] union (live_out[n] - kill[n])
        let old_live_in = live_in
            .insert(
                node,
                union(
                    cfg.gen_node(node),
                    difference(live_out.get(&node).unwrap().clone(), cfg.kill_node(node)),
                ),
            )
            .unwrap_or(HashSet::new());

        // if old_live_in != live_in[n]:
        if &old_live_in != live_in.get(&node).unwrap_or(&HashSet::new()) {
            // add all predecessors of n to work-list
            let predecessors = cfg.pred(node);
            for predecessor in predecessors {
                work_list.push_back(predecessor);
            }
        }
    }

    cfg
}

/*
 * construct_control_flow_graph_lir
 *
 * Given an LIRFunction create a Control Flow Graph of the LIRAssembly.
 * Live In sets are initialized to empty
 */
pub fn construct_control_flow_graph_lir(function: &LIRFunction) -> ControlFlowGraph<LIRAssembly> {
    let mut graph = StableGraph::new();
    let mut labels: HashMap<Label, NodeIndex<DefaultIx>> = HashMap::new();

    let start_node = graph.add_node(Node::Start);
    let end_node = graph.add_node(Node::End);

    // Collect all the labels:
    for (index, assembly_line) in function.instruction_listing.iter().enumerate() {
        match assembly_line {
            LIRAssembly::Label(l) => {
                let n = graph.add_node(Node::Index(index));
                labels.insert(*l, n);
            }
            _ => continue,
        }
    }

    let mut current_node = Some(start_node);

    // Insert All the Nodes
    for (index, assembly_line) in function.instruction_listing.iter().enumerate() {
        match assembly_line {
            LIRAssembly::Label(l) => {
                let node = labels.get(l).unwrap();
                if let Some(cn) = current_node {
                    graph.add_edge(cn, *node, ());
                }
                current_node = Some(*node);
            }
            LIRAssembly::Instruction(LIRInstruction::Jump { to }) => {
                let node = graph.add_node(Node::Index(index));
                if let Some(cn) = current_node {
                    graph.add_edge(cn, node, ());
                }

                let to_node = labels.get(to).unwrap();
                graph.add_edge(node, *to_node, ());

                current_node = None;
            }
            LIRAssembly::Instruction(LIRInstruction::JumpC { to, .. }) => {
                let node = graph.add_node(Node::Index(index));
                if let Some(cn) = current_node {
                    graph.add_edge(cn, node, ());
                }

                let to_node = labels.get(to).unwrap();
                graph.add_edge(node, *to_node, ());

                current_node = Some(node);
            }
            LIRAssembly::Instruction(_) => {
                let node = graph.add_node(Node::Index(index));
                if let Some(cn) = current_node {
                    graph.add_edge(cn, node, ());
                }
                current_node = Some(node)
            }
        }
    }

    if let Some(cn) = current_node {
        graph.add_edge(cn, end_node, ());
    }

    ControlFlowGraph {
        graph,
        data: function
            .instruction_listing
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, _)| (Node::Index(i), HashSet::new()))
            .chain(vec![
                (Node::Start, HashSet::new()),
                (Node::End, HashSet::new()),
            ])
            .collect(),
        instructions: function.instruction_listing.iter().cloned().collect(),
    }
}

/*
 * construct_control_flow_graph_x64s
 *
 * Given an X64SFunction create a Control Flow Graph of the X64SAssembly
 * Live In sets are initialized to empty
 */
pub fn construct_control_flow_graph_x64s(
    function: &X64SFunction,
) -> ControlFlowGraph<X64SAssembly> {
    let mut graph = StableGraph::new();
    let mut labels: HashMap<Label, NodeIndex<DefaultIx>> = HashMap::new();

    let start_node = graph.add_node(Node::Start);
    let end_node = graph.add_node(Node::End);

    // Collect all the labels:
    for (index, assembly_line) in function.body.iter().enumerate() {
        match assembly_line {
            X64SAssembly::Label(l) => {
                let n = graph.add_node(Node::Index(index));
                labels.insert(*l, n);
            }
            _ => continue,
        }
    }

    let mut current_node = Some(start_node);

    // Insert All the Nodes
    for (index, assembly_line) in function.body.iter().enumerate() {
        match assembly_line {
            X64SAssembly::Label(l) => {
                let node = labels.get(l).unwrap();
                if let Some(cn) = current_node {
                    graph.add_edge(cn, *node, ());
                }
                current_node = Some(*node);
            }
            X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::Jmp,
                args: SOperands::One(SOperand::MemoryImm(X64Value::LabelRef(to))),
            }) => {
                let node = graph.add_node(Node::Index(index));
                if let Some(cn) = current_node {
                    graph.add_edge(cn, node, ());
                }

                let to_node = labels.get(to).unwrap();
                graph.add_edge(node, *to_node, ());

                current_node = None;
            }
            X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::Je,
                args: SOperands::One(SOperand::MemoryImm(X64Value::LabelRef(to))),
            })
            | X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::Jne,
                args: SOperands::One(SOperand::MemoryImm(X64Value::LabelRef(to))),
            })
            | X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::Jg,
                args: SOperands::One(SOperand::MemoryImm(X64Value::LabelRef(to))),
            })
            | X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::Jge,
                args: SOperands::One(SOperand::MemoryImm(X64Value::LabelRef(to))),
            })
            | X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::Jl,
                args: SOperands::One(SOperand::MemoryImm(X64Value::LabelRef(to))),
            })
            | X64SAssembly::Instruction(X64SInstruction {
                op_code: X64opCode::Jle,
                args: SOperands::One(SOperand::MemoryImm(X64Value::LabelRef(to))),
            }) => {
                let node = graph.add_node(Node::Index(index));
                if let Some(cn) = current_node {
                    graph.add_edge(cn, node, ());
                }

                let to_node = labels.get(to).unwrap();
                graph.add_edge(node, *to_node, ());

                current_node = Some(node);
            }
            X64SAssembly::Instruction { .. } => {
                let node = graph.add_node(Node::Index(index));
                if let Some(cn) = current_node {
                    graph.add_edge(cn, node, ());
                }
                current_node = Some(node)
            }
        }
    }

    if let Some(cn) = current_node {
        graph.add_edge(cn, end_node, ());
    }

    ControlFlowGraph {
        graph,
        data: function
            .body
            .iter()
            .cloned()
            .enumerate()
            .map(|(i, _)| (Node::Index(i), HashSet::new()))
            .chain(vec![
                (Node::Start, HashSet::new()),
                (Node::End, HashSet::new()),
            ])
            .collect(),
        instructions: function.body.iter().cloned().collect(),
    }
}

pub use crate::common::InfixSourceOp;

use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Program {
    pub dec_list: VecDeque<TopLevelDec>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TopLevelDec {
    TyDecArray {
        new_type: String,
        element_type: String,
    },
    TyDecRecord {
        new_type: String,
        field_decs: VecDeque<(String, String)>,
    },
    FunDec {
        name: String,
        return_type: String,
        args: VecDeque<(String, String)>,
        body: Box<Exp>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Exp {
    Break,
    IntLit {
        value: i32,
    },
    StringLit {
        value: String,
    },
    LValue {
        lvalue: LValue,
    },
    Sequence {
        sequence: VecDeque<Exp>,
    },
    Negate {
        exp: Box<Exp>,
    },
    Infix {
        left: Box<Exp>,
        op: InfixSourceOp,
        right: Box<Exp>,
    },
    ArrayCreate {
        type_id: String,
        length: Box<Exp>,
        inital_value: Box<Exp>,
    },
    RecordCreate {
        type_id: String,
        fields: VecDeque<(String, Exp)>,
    },
    Assign {
        left: LValue,
        right: Box<Exp>,
    },
    IfThenElse {
        if_exp: Box<Exp>,
        then_exp: Box<Exp>,
        else_exp: Box<Exp>,
    },
    IfThen {
        if_exp: Box<Exp>,
        then_exp: Box<Exp>,
    },
    While {
        while_exp: Box<Exp>,
        do_exp: Box<Exp>,
    },
    For {
        id: String,
        for_exp: Box<Exp>,
        to_exp: Box<Exp>,
        do_exp: Box<Exp>,
    },
    Let {
        let_exp: VecDeque<Dec>,
        in_exp: Box<Exp>,
    },
    Call {
        function_name: String,
        args: VecDeque<Exp>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LValue {
    Id { name: String },
    Subscript { array: Box<LValue>, index: Box<Exp> },
    FieldExp { record: Box<LValue>, field: String },
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Dec {
    VarDec {
        name: String,
        value_type: String,
        value: Exp,
    },
}

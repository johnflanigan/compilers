use crate::check_type::TypeError;
use crate::common::{InfixSourceOp, Label, LabelGenerator, Symbol, SymbolGenerator};

use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CheckedProgram {
    pub function_symbols: HashMap<Label, FunctionType>,
    pub symbol_table: HashMap<Symbol, TypeId>,
    pub types: HashMap<TypeId, Type>,
    pub gen_sym: SymbolGenerator,
    pub gen_label: LabelGenerator,
    pub dec_list: VecDeque<CheckedTopLevelDec>,
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub enum CheckedTopLevelDec {
    FunDec {
        name: Label,
        args: VecDeque<Symbol>,
        body: Box<CheckedExp>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub enum CheckedExp {
    Break,
    IntLit {
        value: i32,
    },
    StringLit {
        value: String,
    },
    LValue {
        lvalue: CheckedLValue,
    },
    Sequence {
        sequence: VecDeque<CheckedExp>,
    },
    Negate {
        exp: Box<CheckedExp>,
    },
    Infix {
        left: Box<CheckedExp>,
        op: InfixSourceOp,
        right: Box<CheckedExp>,
    },
    ArrayCreate {
        length: Box<CheckedExp>,
        initial_value: Box<CheckedExp>,
    },
    RecordCreate {
        fields: VecDeque<(String, CheckedExp)>,
    },
    Assign {
        left: CheckedLValue,
        right: Box<CheckedExp>,
    },
    IfThenElse {
        if_exp: Box<CheckedExp>,
        then_exp: Box<CheckedExp>,
        else_exp: Box<CheckedExp>,
    },
    IfThen {
        if_exp: Box<CheckedExp>,
        then_exp: Box<CheckedExp>,
    },
    While {
        while_exp: Box<CheckedExp>,
        do_exp: Box<CheckedExp>,
    },
    For {
        id: Symbol,
        for_exp: Box<CheckedExp>,
        to_exp: Box<CheckedExp>,
        do_exp: Box<CheckedExp>,
    },
    Let {
        let_exp: VecDeque<CheckedDec>,
        in_exp: Box<CheckedExp>,
    },
    Call {
        function_name: Label,
        args: VecDeque<CheckedExp>,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub enum CheckedLValue {
    Id {
        name: Symbol,
    },
    Subscript {
        array: Box<CheckedLValue>,
        index: Box<CheckedExp>,
    },
    FieldExp {
        record: Box<CheckedLValue>,
        field: String,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub enum CheckedDec {
    VarDec { name: Symbol, value: CheckedExp },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct TypeId(u64);

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum Type {
    Void,
    Str,
    Int,
    Record(Vec<(String, TypeId)>),
    Array(TypeId),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FunctionType {
    pub return_type: TypeId,
    pub arguments: Vec<(Symbol, TypeId)>,
}

pub type B = bool;

/****** Type Checking/Lowering Support  *******/
#[derive(Debug, Clone)]
pub struct GlobalTypeInfo {
    pub function_symbols: HashMap<Label, FunctionType>,
    pub symbol_table: HashMap<Symbol, TypeId>,
    pub types: HashMap<TypeId, Type>,
    pub gen_type: GenerateTypeId,
    pub gen_sym: SymbolGenerator,
    pub gen_label: LabelGenerator,
}

// Type Checking/Lowering Utility Code:
impl GlobalTypeInfo {
    pub fn new_type_id(&mut self, type_: Type) -> TypeId {
        let new_type_id = self.gen_type.new_type_id();
        self.types.insert(new_type_id, type_);
        new_type_id
    }

    pub fn insert_gamma(&mut self, id: Symbol, type_id: TypeId) -> Result<(), TypeError> {
        if self.symbol_table.contains_key(&id) {
            Err(TypeError("Duplicate Symbol"))
        } else {
            self.symbol_table.insert(id, type_id);
            Ok(())
        }
    }

    pub fn lookup_gamma(&self, name: Symbol) -> Option<TypeId> {
        self.symbol_table.get(&name).copied()
    }

    pub fn lookup_c(&self, name: TypeId) -> Option<Type> {
        self.types.get(&name).cloned()
    }

    pub fn lookup_f(&self, name: &Label) -> Option<FunctionType> {
        self.function_symbols.get(name).cloned()
    }

    pub fn insert_f(&mut self, id: &Label, fn_type: FunctionType) -> Result<(), TypeError> {
        if self.function_symbols.contains_key(id) {
            Err(TypeError("Duplicate Label"))
        } else {
            self.function_symbols.insert(*id, fn_type);
            Ok(())
        }
    }
}

#[derive(Debug, Clone)]
pub struct GenerateTypeId(u64);

impl GenerateTypeId {
    pub fn new() -> GenerateTypeId {
        GenerateTypeId(0)
    }

    pub fn new_type_id(&mut self) -> TypeId {
        let uid = self.0;
        self.0 = uid + 1;
        TypeId(uid)
    }
}

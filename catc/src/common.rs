use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::iter::Iterator;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Label {
    Uid(u64),
    Allocate,
    AllocateAndMemset,
    PrintlnInt,
    PrintlnString,
    PrintInt,
    PrintString,
    Main,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct LabelGenerator {
    next_uid: u64,
}

impl LabelGenerator {
    pub fn new() -> Self {
        LabelGenerator { next_uid: 0 }
    }
    pub fn new_label(&mut self) -> Label {
        let uid = self.next_uid;
        self.next_uid += 1;
        Label::Uid(uid)
    }
    pub fn from_vec(vec: &Vec<Label>) -> LabelGenerator {
        LabelGenerator {
            next_uid: vec
                .iter()
                .filter_map(|l| match l {
                    Label::Uid(uid) => Some(*uid),
                    _ => None,
                })
                .max_by(|l1, l2| l1.cmp(l2))
                .unwrap_or(0),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub struct Comparison {
    pub c: ComparisonType,
    pub left: Symbol,
    pub right: Symbol,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub enum ComparisonType {
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanEqual,
    LessThanEqual,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Serialize, Deserialize)]
pub enum InfixOp {
    Multiply,
    Divide,
    Add,
    Subtract,
    And,
    Or,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
pub struct Symbol {
    uid: u64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SymbolGenerator {
    next_uid: u64,
}

impl SymbolGenerator {
    pub fn new() -> Self {
        SymbolGenerator { next_uid: 0 }
    }
    pub fn new_symbol(&mut self) -> Symbol {
        let uid = self.next_uid;
        self.next_uid += 1;
        Symbol { uid }
    }
    pub fn from_vec(vec: &Vec<Symbol>) -> SymbolGenerator {
        SymbolGenerator {
            next_uid: vec
                .iter()
                .map(|sym| sym.uid)
                .max_by(|l1, l2| l1.cmp(&l2))
                .unwrap_or(0),
        }
    }
}

use std::fmt;

impl fmt::Display for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Label::Uid(u) => write!(f, "L{}", u),
            Label::Allocate => write!(f, "allocate"),
            Label::AllocateAndMemset => write!(f, "allocate_and_memset"),
            Label::PrintlnInt => write!(f, "_print_line_int"),
            Label::PrintlnString => write!(f, "_print_line_string"),
            Label::PrintInt => write!(f, "_print_int"),
            Label::PrintString => write!(f, "_print_string"),
        }
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "S{}", self.uid)
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Deserialize, Serialize)]
pub enum InfixSourceOp {
    Multiply,
    Divide,
    Add,
    Subtract,
    Equal,
    NotEqual,
    GreaterThan,
    LessThan,
    GreaterThanEqual,
    LessThanEqual,
    And,
    Or,
}

impl TryInto<InfixOp> for InfixSourceOp {
    type Error = ();

    fn try_into(self) -> Result<InfixOp, Self::Error> {
        match self {
            Self::Multiply => Ok(InfixOp::Multiply),
            Self::Divide => Ok(InfixOp::Divide),
            Self::Add => Ok(InfixOp::Add),
            Self::Subtract => Ok(InfixOp::Subtract),
            Self::Equal => Err(()),
            Self::NotEqual => Err(()),
            Self::GreaterThan => Err(()),
            Self::LessThan => Err(()),
            Self::GreaterThanEqual => Err(()),
            Self::LessThanEqual => Err(()),
            Self::And => Ok(InfixOp::And),
            Self::Or => Ok(InfixOp::Or),
        }
    }
}

impl TryInto<ComparisonType> for InfixSourceOp {
    type Error = ();

    fn try_into(self) -> Result<ComparisonType, Self::Error> {
        match self {
            Self::Multiply => Err(()),
            Self::Divide => Err(()),
            Self::Add => Err(()),
            Self::Subtract => Err(()),
            Self::Equal => Ok(ComparisonType::Equal),
            Self::NotEqual => Ok(ComparisonType::NotEqual),
            Self::GreaterThan => Ok(ComparisonType::GreaterThan),
            Self::LessThan => Ok(ComparisonType::LessThan),
            Self::GreaterThanEqual => Ok(ComparisonType::GreaterThanEqual),
            Self::LessThanEqual => Ok(ComparisonType::LessThanEqual),
            Self::And => Err(()),
            Self::Or => Err(()),
        }
    }
}

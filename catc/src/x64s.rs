use crate::x64::{Operand, X64Register, X64Value, X64opCode};

use serde::{Deserialize, Serialize};

use crate::common::{Label, Symbol};

use std::collections::HashMap;

use std::convert::TryInto;

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum SOperand {
    Symbol(Symbol),
    MemorySym(Symbol),
    Immediate(X64Value),
    Register(X64Register),
    MemoryImm(X64Value),
    MemoryReg(X64Register),
    MemoryOffset(X64Value, X64Register),
    MemoryScaledIndexed(X64Value, X64Register, u8, X64Register),
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum SOperands {
    Zero,
    One(SOperand),
    Two(SOperand, SOperand),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct X64SInstruction {
    pub op_code: X64opCode,
    pub args: SOperands,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum X64SAssembly {
    Label(Label),
    Instruction(X64SInstruction),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct X64SFunction {
    pub body: Vec<X64SAssembly>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct X64SProgram {
    pub main_function: X64SFunction,
    pub other_functions: HashMap<Label, X64SFunction>,
    pub string_literals: HashMap<Label, String>,
}

impl TryInto<Operand> for SOperand {
    type Error = ();

    fn try_into(self) -> Result<Operand, Self::Error> {
        match self {
            SOperand::Symbol(_) => Err(()),
            SOperand::MemorySym(_) => Err(()),
            SOperand::Immediate(v) => Ok(Operand::Immediate(v)),
            SOperand::Register(r) => Ok(Operand::Register(r)),
            SOperand::MemoryImm(v) => Ok(Operand::MemoryImm(v)),
            SOperand::MemoryReg(r) => Ok(Operand::MemoryReg(r)),
            SOperand::MemoryOffset(v, r) => Ok(Operand::MemoryOffset(v, r)),
            SOperand::MemoryScaledIndexed(v, r1, u8, r2) => {
                Ok(Operand::MemoryScaledIndexed(v, r1, u8, r2))
            }
        }
    }
}

use std::fmt;

impl fmt::Display for X64SProgram {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Basic main only assembly generation:
        let mut program: String = format!("main:\n{}\n", self.main_function);

        for (k, v) in self.other_functions.iter() {
            program.push_str(format!("{}:\n{}", k, v).as_str());
        }

        for (k, v) in self.string_literals.iter() {
            program.push_str(format!("{}:\t.string \"{}\"\n", k, v).as_str());
        }

        write!(f, "{}", program)
    }
}

impl fmt::Display for SOperand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SOperand::Symbol(s) => write!(f, "{}", s),
            SOperand::MemorySym(s) => write!(f, "*{}", s),
            SOperand::Immediate(i) => write!(f, "${}", i),
            SOperand::Register(r) => write!(f, "{}", r),
            SOperand::MemoryImm(v) => write!(f, "{}", v),
            SOperand::MemoryReg(r) => write!(f, "({})", r),
            SOperand::MemoryOffset(offset, r) => write!(f, "{}({})", offset, r),
            SOperand::MemoryScaledIndexed(offset, r, scale, index) => {
                write!(f, "{}({}, {}, {})", offset, r, index, scale)
            }
        }
    }
}

impl fmt::Display for SOperands {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SOperands::Zero => write!(f, ""),
            SOperands::One(o) => write!(f, "{}", o),
            SOperands::Two(a, b) => write!(f, "{}, {}", a, b),
        }
    }
}

impl fmt::Display for X64SInstruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\t{}", self.op_code, self.args)
    }
}

impl fmt::Display for X64SAssembly {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            X64SAssembly::Label(l) => write!(f, "{}:", l),
            X64SAssembly::Instruction(i) => write!(f, "\t{}", i),
        }
    }
}

impl fmt::Display for X64SFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.body
                .iter()
                .map(|assem| {
                    let mut assem = assem.to_string();
                    assem.push_str("\n");
                    assem
                })
                .collect::<String>()
        )
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum AsmOp {
    Add(AsmOperand, AsmOperand),
    Sub(Register, AsmOperand),

    Mul(Register, AsmOperand),
    Div(Register, AsmOperand),
    Mod(Register, AsmOperand),

    Mov(AsmOperand, AsmOperand),
    Push(AsmOperand),
    Pop(AsmOperand),
    Out,

    Label(String),
    Loop(String),

    Cmp(AsmOperand, AsmOperand),
    Je(String),
    Ja(String),
    Jb(String),
    Jne(String),
    Jae(String),
    Jbe(String),
    Jmp(String),

    Nop,
}

use std::fmt::{Display, Formatter, Result};
impl Display for AsmOp {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use self::AsmOp::*;
        match self {
            &Cmp(ref l, ref r) => write!(f, "cmp {}, {}\n", l, r),
            &Je(ref label) => write!(f, "je {}\n", label),
            &Ja(ref label) => write!(f, "ja {}\n", label),
            &Jb(ref label) => write!(f, "jb {}\n", label),
            &Jne(ref label) => write!(f, "jne {}\n", label),
            &Jae(ref label) => write!(f, "jae {}\n", label),
            &Jbe(ref label) => write!(f, "jbe {}\n", label),
            &Jmp(ref label) => write!(f, "jmp {}\n", label),
            &Out => {
                write!(f,
                       r"
;printing variable
push r8
push rcx
mov r10, print
invoke r10, qword [r8]
pop rcx
pop r8

")
            }
            _ => unimplemented!(),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum AsmOperand {
    RegisterOperand(Register),
    Value(i32),
    Memory(u16),
    MemoryRegister(Register),
}

impl Display for AsmOperand {
    fn fmt(&self, f: &mut Formatter) -> Result {
        use self::AsmOperand::*;
        match self {
            &RegisterOperand(ref dest) => write!(f, "{}", dest),
            &Value(ref dest) => write!(f, "{}", dest),
            &Memory(ref mem) => write!(f, "[{}]", mem),
            &MemoryRegister(ref mem) => write!(f, "[{}]", mem),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Register {
    RAX,
    RBX,
    RCX,
    RDX,
    R8,
    R15,
}

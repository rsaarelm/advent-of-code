use std::str::FromStr;

use serde_with::DeserializeFromStr;

use crate::prelude::*;

#[derive(Copy, Clone, Debug, DeserializeFromStr)]
/// An assembly code operand that can be either a constant or a register name.
pub enum Operand {
    Int(i64),
    Reg(char),
}

use Operand::*;

impl Operand {
    pub fn val(&self, mem: &[i64]) -> i64 {
        match self {
            Int(n) => *n,
            Reg(c) => mem[reg(*c)],
        }
    }
}

impl FromStr for Operand {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<i64>() {
            Ok(n) => Ok(Int(n)),
            _ => Ok(Reg(s.chars().next().unwrap())),
        }
    }
}

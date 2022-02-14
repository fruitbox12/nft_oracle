//! Instruction types
use solana_program::program_error::ProgramError;


use crate::{
    error::TokenError,
};
use std::convert::TryInto;


pub struct ProcessUpdate {
    pub amount: u64

}
pub struct ProcessCollector {
    pub number: u64

}

pub enum TokenInstruction {
    ProcessCollector(ProcessCollector),
    ProcessUpdate(ProcessUpdate),
}

impl TokenInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        use TokenError::InvalidInstruction;
        let (&tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
        Ok(match tag {
            0 => {             
                let (number, _rest) = rest.split_at(8);
                let number = number
                    .try_into()
                    .map(u64::from_le_bytes)
                    .or(Err(InvalidInstruction))?;                
                Self::ProcessCollector(ProcessCollector{number})
            }
            1 => {
                let (amount, _rest) = rest.split_at(8);
                let amount = amount
                    .try_into()
                    .map(u64::from_le_bytes)
                    .or(Err(InvalidInstruction))?;                
                Self::ProcessUpdate(ProcessUpdate{amount})
            }
            _ => return Err(TokenError::InvalidInstruction.into()),
        })
    }
}

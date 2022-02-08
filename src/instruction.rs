//! Instruction types
use solana_program::program_error::ProgramError;
use std::convert::TryInto;
use crate::error::TokenError;

pub struct ProcessWhitelist ;
pub struct ProcessUpdate {
    pub amount: u64

}

pub enum TokenInstruction {
    ProcessWhitelist(ProcessWhitelist),
    ProcessUpdate(ProcessUpdate),
}

impl TokenInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        use TokenError::InvalidInstruction;
        let (&tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
        Ok(match tag {
            0 => {             
                Self::ProcessWhitelist(ProcessWhitelist)
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

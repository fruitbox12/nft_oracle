//! Instruction types
use solana_program::program_error::ProgramError;

use {borsh::{BorshDeserialize}};

use crate::{
    error::TokenError,
    state::CREATORS,
};
use std::convert::TryInto;


pub struct ProcessUpdate {
    pub amount: u64

}

pub enum TokenInstruction {
    ProcessWhitelist{
        nft_creator: CREATORS,
    },
    ProcessUpdate(ProcessUpdate),
}

impl TokenInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        use TokenError::InvalidInstruction;
        let (&tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
        Ok(match tag {
            0 => {             
                Self::ProcessWhitelist{nft_creator:CREATORS::try_from_slice(rest)?}
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

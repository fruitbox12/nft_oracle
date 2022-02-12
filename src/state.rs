///into state.rs
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info:: AccountInfo,
    program_error::ProgramError,
    pubkey::Pubkey,
    borsh::try_from_slice_unchecked,}; 
//price state
#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct Price {
     
    pub creator: CREATORS,
    pub admin_account: Pubkey,
    pub update_time: u64,
    pub price:u64,
}
#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct CREATORS{
    pub address: Vec<Pubkey>,
}
impl Price {
    pub fn from_account(account:&AccountInfo)-> Result<Price, ProgramError> {
            let md: Price =try_from_slice_unchecked(&account.data.borrow_mut())?;
            Ok(md)
    }
}

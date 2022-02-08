///into state.rs
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
//deposit tokens
#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug, Clone)]
pub struct Price {
     
    pub admin_account: Pubkey,
    pub creator_1: Pubkey,
    pub creator_2: Pubkey,
    pub creator_3: Pubkey,
    pub update_time: u64,
    pub price:u64,
}


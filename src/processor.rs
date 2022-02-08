//! Program state processor

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    sysvar::{Sysvar,rent::Rent,clock::Clock},
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
};

use crate::{
    instruction::{ProcessUpdate,TokenInstruction},
    state::Price
};
use std::{
    str::FromStr
  };
pub struct Processor {}

impl Processor {
    pub fn process_whitelist(program_id: &Pubkey,accounts: &[AccountInfo],creator:Price) -> ProgramResult 
    {
        //executed once
        let account_info_iter = &mut accounts.iter();
        let admin_account = next_account_info(account_info_iter)?; // admin who updates the price
        let system_program = next_account_info(account_info_iter)?;
        let pda_data =next_account_info(account_info_iter)?; //account to save data // this account gives the price feed
     
        //Was the transaction signed by admin account's private key
        if !admin_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        let admin_key="some string"; //always fixed
        let admin_key = Pubkey::from_str( admin_key ).unwrap();

          //Was the transaction updated by admin account
        if *admin_account.key !=admin_key
        {
            return Err(ProgramError::MissingRequiredSignature);
        }


        let rent = Rent::get()?;
        let size: u64=std::mem::size_of::<Price>() as u64 + 180;
        let transfer_amount =  rent.minimum_balance (size as usize);
       //creating the data feed account
        invoke(
            &system_instruction::create_account(
                admin_account.key,
                pda_data.key,
                transfer_amount,
                size,
                program_id,
            ),
            &[
                admin_account.clone(),
                pda_data.clone(),
                system_program.clone(),
            ],
        )?;
        
        let mut pda_start = Price::from_account(pda_data)?;
        
        pda_start.creator = creator.creator;
        pda_start.admin_account=*admin_account.key;
        pda_start.serialize(&mut *pda_data.data.borrow_mut())?;
        
        Ok(())
    }
    
    pub fn update_price(program_id: &Pubkey,accounts: &[AccountInfo],amount:u64)->ProgramResult
    {  
        //changing the price

        let account_info_iter = &mut accounts.iter();
        let admin_account = next_account_info(account_info_iter)?; // admin who updates the price
        let pda_data =next_account_info(account_info_iter)?; //account to save data 


        if !admin_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        let admin_key="some string"; //always fixed
        let admin_key = Pubkey::from_str( admin_key ).unwrap();

          //Was the transaction updated by admin account
        if *admin_account.key !=admin_key
        {
            return Err(ProgramError::MissingRequiredSignature);
        }
        let mut pda_update = Price::try_from_slice(&pda_data.data.borrow())?;
        let mut k = 0; 

        //verifying the collection
        for i in 0..pda_update.creator.len()
        {
            let creator = next_account_info(account_info_iter)?;
            if *creator.key == pda_update.creator[i].address
            {
                k+=1;
            }
        }
        // if not verified return error
        if k < pda_update.creator.len()
        {
            return Err(ProgramError::MissingRequiredSignature);
        }

        if pda_data.owner != program_id
        {
            return Err(ProgramError::MissingRequiredSignature);
        } 

        //update the price
        pda_update.price= amount;
        let now:u64 = Clock::get()?.unix_timestamp as u64;
        pda_update.update_time= now;

     
        pda_update.serialize(&mut &mut pda_data.data.borrow_mut()[..])?;
        Ok(())

    }

        
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
        let instruction = TokenInstruction::unpack(input)?;
        match instruction {
            TokenInstruction::ProcessWhitelist{creator} => {
                msg!("Instruction: Whitelisting Collection");
                Self::process_whitelist(program_id, accounts,creator)
            }
            TokenInstruction::ProcessUpdate(ProcessUpdate{ amount }) => {
                msg!("Instruction: Updating Price");
                Self::update_price(program_id, accounts, amount)
            }
        }
    }
}

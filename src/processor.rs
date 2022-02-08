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
    instruction::{ProcessWhitelist,ProcessUpdate,TokenInstruction},
    state::Price,
};
use std::{
    str::FromStr
  };
pub struct Processor {}

impl Processor {
    pub fn process_whitelist(program_id: &Pubkey,accounts: &[AccountInfo]) -> ProgramResult {


        let account_info_iter = &mut accounts.iter();
        let admin_account = next_account_info(account_info_iter)?; // admin who updates the price
        let system_program = next_account_info(account_info_iter)?;
        let creator_1=next_account_info(account_info_iter)?;
        let creator_2=next_account_info(account_info_iter)?;
        let creator_3=next_account_info(account_info_iter)?;
        let pda_data =next_account_info(account_info_iter)?; //account to save data 
     
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
        let creator_1_key=Pubkey::from_str("1").unwrap();
        let creator_2_key=Pubkey::from_str("1").unwrap();
        let creator_3_key=Pubkey::from_str("1").unwrap();

        //verifying the collection
        if *creator_1.key != creator_1_key && *creator_2.key != creator_2_key && *creator_3.key != creator_3_key
        {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let rent = Rent::get()?;
        let transfer_amount =  rent.minimum_balance (std::mem::size_of::<Price>());
        // Sending transaction fee to recipient. So, he can withdraw the streamed fund
        invoke(
            &system_instruction::create_account(
                admin_account.key,
                pda_data.key,
                transfer_amount,
                std::mem::size_of::<Price>() as u64,
                program_id,
            ),
            &[
                admin_account.clone(),
                pda_data.clone(),
                system_program.clone(),
            ],
        )?;
        
        let mut pda_start = Price::try_from_slice(&pda_data.data.borrow())?;
        

        pda_start.admin_account = *admin_account.key;
        pda_start.creator_1 = *creator_1.key;
        pda_start.creator_2 = *creator_2.key;
        pda_start.creator_3 = *creator_3.key;
        
        pda_start.serialize(&mut &mut pda_data.data.borrow_mut()[..])?;
        
        Ok(())
    }
    
    pub fn update_price(program_id: &Pubkey,accounts: &[AccountInfo],amount:u64)->ProgramResult
    {  
        let account_info_iter = &mut accounts.iter();
        let admin_account = next_account_info(account_info_iter)?; // admin who updates the price
        let creator_1=next_account_info(account_info_iter)?;
        let creator_2=next_account_info(account_info_iter)?;
        let creator_3=next_account_info(account_info_iter)?;
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
        let creator_1_key=Pubkey::from_str("1").unwrap();
        let creator_2_key=Pubkey::from_str("1").unwrap();
        let creator_3_key=Pubkey::from_str("1").unwrap();

        //verifying the collection
        if *creator_1.key != creator_1_key && *creator_2.key != creator_2_key && *creator_3.key != creator_3_key
        {
            return Err(ProgramError::MissingRequiredSignature);
        }
        if pda_data.owner != program_id
        {
            return Err(ProgramError::MissingRequiredSignature);
        } 
        let mut pda_update = Price::try_from_slice(&pda_data.data.borrow())?;
        pda_update.price= amount;
        let now:u64 = Clock::get()?.unix_timestamp as u64;
        pda_update.update_time= now;

     
        pda_update.serialize(&mut &mut pda_data.data.borrow_mut()[..])?;
     


    Ok(())
       

    }

        
    pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], input: &[u8]) -> ProgramResult {
        let instruction = TokenInstruction::unpack(input)?;
        match instruction {
            TokenInstruction::ProcessWhitelist(ProcessWhitelist) => {
                msg!("Instruction: Whitelisting Collection");
                Self::process_whitelist(program_id, accounts)
            }
            TokenInstruction::ProcessUpdate(ProcessUpdate{ amount }) => {
                msg!("Instruction: Updating Price");
                Self::update_price(program_id, accounts, amount)
            }
        }
    }
}

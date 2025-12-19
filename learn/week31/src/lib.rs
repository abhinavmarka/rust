use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshDeserialize, BorshSerialize, Debug, PartialEq)]
pub enum Instruction {
    Increment,
    Decrement,
    Reset,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct Counter {
    pub count: u32,
}

impl Counter {
    pub fn new() -> Self {
        Self { count: 0 }
    }

    pub fn increment(&mut self) {
        self.count = self.count.saturating_add(1);
    }

    pub fn decrement(&mut self) {
        self.count = self.count.saturating_sub(1);
    }

    pub fn reset(&mut self) {
        self.count = 0;
    }
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Counter program entry point");                                                                                            
    
    let instruction = Instruction::try_from_slice(instruction_data)
    .map_err(|_| ProgramError::InvalidInstructionData)?;
    
    let accounts_iter = &mut accounts.iter();
    let counter_account = next_account_info(accounts_iter)?;
    
    if counter_account.owner != program_id {
        return Err(ProgramError::IncorrectProgramId);
    }
    
    let mut counter = Counter::try_from_slice(&counter_account.data.borrow())?;
    
    match instruction {
        Instruction::Increment => {
            msg!("Incrementing counter from {}", counter.count);
            counter.increment();
        }
        Instruction::Decrement => {
            msg!("Decrementing counter from {}", counter.count);
            counter.decrement();
        }
        Instruction::Reset => {
            msg!("Resetting counter");
            counter.reset();
        }
    }
    
    msg!("New counter value: {}", counter.count);
    counter.serialize(&mut &mut counter_account.data.borrow_mut()[..])?;
    
    Ok(())
}
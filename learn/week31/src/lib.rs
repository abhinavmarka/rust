use solana_program::{
    account_info::(AccountInfo, next account info),
    entrypoint::ProgramResult, entrypoint,
    pubkey::Pubkey,

};

#[derive{BorshDeserialize, BorshSerialize}]

enum Instruction {
    Increment,
    Decrement,
}

#[derive(BorshDeserialize, BorshSerialize)]
struct Counter {
    count: u32,
}

entrypoint!{counter_contract};

pub fn counter_contract(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
    ) -> ProgramResult {
        Ok(())
    } -> ProgramResult {
        let acc: &AccountInfo<'_, Counter> = next_account_info(&mut accounts.iter())?;
        match instruction_data[0] {
            0 => counter.count += 1,
            1 => counter.count -= 1,
            _ => return Err(ErrorCode::InvalidInstruction.into()),
        }
        Ok(())
    }
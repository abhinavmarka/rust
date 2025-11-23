use solana_program::{
    account_info::(AccountInfo, next account info),
    entrypoint::ProgramResult, entrypoint,
    pubkey::Pubkey,

};

entrypoint!{counter_contract};

pub fn counter_contract(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8]
    ) -> ProgramResult {
        Ok(())
    }
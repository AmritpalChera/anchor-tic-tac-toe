use anchor_lang::prelude::*;

declare_id!("8yKkFNF7AmQivwb7NmLY3granCvv9dujhmTaz7nRA57p");

#[program]
pub mod tic_tac_toe {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

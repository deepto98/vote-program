use anchor_lang::prelude::*;

declare_id!("3DV5U3QhmL5oP2W1W69HzwxH9pMvjWqHprd8gdQTtw9J");

#[program]
pub mod vote_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

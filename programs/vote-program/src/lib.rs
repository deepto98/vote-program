use anchor_lang::prelude::*;

declare_id!("3DV5U3QhmL5oP2W1W69HzwxH9pMvjWqHprd8gdQTtw9J");

#[program]
pub mod vote_program {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, _url:String) -> Result<()> {
        ctx.accounts.initialize(&ctx.bumps)?;    //initialize acc with bumps
        Ok(())
    }
}

//   Account specs for contexts
#[derive(Accounts)]
#[instruction(_url: String)]
// 1. Initialize Context
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>, //who pays for the account creation
    #[account(
        init, //init constraint because the account doesn't exist, has to be init
        payer=payer,
        seeds=[_url.as_bytes().as_ref()],     //url is passed from instruction
        bump,//find canonical bump
        space = VoteState::INIT_SPACE, 
    )]
    pub vote_state: Account<'info, VoteState>,// the vote state PDA account
    pub system_program:Program<'info,System>,
}

// Implementing functionality
impl<'info>Initialize<'info>{
    pub fn initialize(&mut self,bumps:InitializeBumps){//all bumps found are stored in InitializeBumps
        self.vote_state.score = 0;
        self.vote_state.bump=bumps.vote_state;
    }
}

// All custom accounts - PDAs
#[account]
#[derive(InitSpace)]
pub struct VoteState {
    pub score: i64,
    pub bump: u8,
}

impl Space for VoteState {
    const INIT_SPACE: usize = 8 + 8 + 1;
}

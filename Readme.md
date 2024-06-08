# Vote Program
The idea is to be able to create URLs and allow people to upvote and downvote it 

anchor init vote-program

1. Accounts we need:
```
#[account]
pub struct VoteState {
    pub score: i64,
    pub bump: u8,
}

impl Space for VoteState {
    const INIT_SPACE: usize = 8 + 8 + 1;
}

```
The initial 8 bytes is for the Anchor discriminator which is used to index our account

2. The Initialize Context:
```
#[derive(Accounts)]
#[instruction(_url: String)]
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
```
Here we implemented the space trait ourselves. TO automate it:
```
#[account]
#[derive(InitSpace)]
pub struct VoteState {
    pub score: i64,
    pub bump: u8,
}
```
But this excludes the anchor discriminator, so we need to add it manually.

```
pub struct Initialize<'info> {
     ....
    #[account(
        init, //init constraint because the account doesn't exist, has to be init
        payer=payer,
        seeds=[_url.as_bytes().as_ref()],     //url is passed from instruction
        bump,//find canonical bump
        space = 8 + VoteState::INIT_SPACE, 
    )]
    pub vote_state: Account<'info, VoteState>,
    ...
```
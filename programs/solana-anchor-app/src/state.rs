
use anchor_lang::prelude::*;

use crate::constants::*;




#[account]
pub struct NewAccount{
    pub name:String,
    pub symbol:String,
    pub decimals:u32
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /*
    Discriminator: 8 bytes
    Fixed size fields: 4 bytes
    String field: 4 (length prefix) + 100 (content) = 104 bytes
     */
    #[account(init, payer = signer, space = 8 + 4 + 4 + 100, seeds = [SEED.as_bytes()],bump)]
    pub new_account: Account<'info, NewAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>
}


#[derive(Accounts)]
pub struct SetAccount<'info> {

    
    #[account(
        mut,
        seeds = [SEED.as_bytes()],
        bump,
    )]
    pub master_acc: Account<'info, NewAccount>,

}


#[derive(Accounts)]
pub struct GetTime{}
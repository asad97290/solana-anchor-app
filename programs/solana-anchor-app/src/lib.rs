use anchor_lang::prelude::*;

declare_id!("JDwTaScuGrwKLXgoRqts9CVongoEZX8s6E8hpqWEr3AK");

#[program]
pub mod solana_anchor_app {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.new_account.data = 1;
        ctx.accounts.new_account.text = "Hello".to_owned();
        msg!("data {}",ctx.accounts.new_account.data);
        msg!("text {}",ctx.accounts.new_account.text);
        Ok(())
    }

    pub fn initialize2(ctx: Context<GetAccount>,_data:u32) -> Result<()> {
        ctx.accounts.master.data = _data;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    /*
    Discriminator: 8 bytes
    Fixed size fields: 4 bytes
    String field: 4 (length prefix) + 100 (content) = 104 bytes
     */
    #[account(init, payer = signer, space = 8 + 4 + 4 + 100, seeds = ["master".as_bytes()],bump)]
    pub new_account: Account<'info, NewAccount>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>
}
#[account]
pub struct NewAccount{
    pub data:u32,
    pub text:String
}



#[derive(Accounts)]
pub struct GetAccount<'info> {
    
    #[account(
        mut,
        seeds = ["master".as_bytes()],
        bump,
    )]
    pub master: Account<'info, NewAccount>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}
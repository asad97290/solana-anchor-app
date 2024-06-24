use anchor_lang::prelude::*;

declare_id!("JDwTaScuGrwKLXgoRqts9CVongoEZX8s6E8hpqWEr3AK");

#[program]
pub mod solana_anchor_app {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>,_name:String,_symbol:String,_decimals:u32) -> Result<()> {
        ctx.accounts.new_account.name = _name;
        ctx.accounts.new_account.symbol = _symbol;
        ctx.accounts.new_account.decimals = _decimals;
        Ok(())
    }

    pub fn change_name_symbol(ctx: Context<SetAccount>,_name:String,_symbol:String) -> Result<()> {
        ctx.accounts.master.name = _name;
        ctx.accounts.master.symbol = _symbol;
        Ok(())
    }

    pub fn block_timestamp(_ctx: Context<GetTime>) -> Result<i64> {
        let clock: Clock = Clock::get()?;
        
        Ok(clock.unix_timestamp)
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
    pub name:String,
    pub symbol:String,
    pub decimals:u32
}



#[derive(Accounts)]
pub struct SetAccount<'info> {

    
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


#[derive(Accounts)]
pub struct GetTime{}
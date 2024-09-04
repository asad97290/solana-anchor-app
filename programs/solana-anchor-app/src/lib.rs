pub mod constants; 
pub mod state;

use anchor_lang::prelude::*;
use state::*;

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
        ctx.accounts.master_acc.name = _name;
        ctx.accounts.master_acc.symbol = _symbol;
        Ok(())
    }

    pub fn block_timestamp(_ctx: Context<GetTime>) -> Result<i64> {
        let clock: Clock = Clock::get().unwrap();
        
        Ok(clock.unix_timestamp)
    }


}



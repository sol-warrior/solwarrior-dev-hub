#![allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

declare_id!("F5S9dz5osJ4Syqd1cmyjwv1odoTzb2pji4EPG4MWY1sb");

#[program]
pub mod store_name_onchain {
    use super::*;

    pub fn initialize(ctx: Context<InitializeName>, name: String) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);

        ctx.accounts.name_store.name = name;
        ctx.accounts.name_store.bump = ctx.bumps.name_store;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeName<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(init, payer = user,seeds = [b"name", user.key().as_ref()],space = 8 + OnchainName::INIT_SPACE, bump)]
    pub name_store: Account<'info, OnchainName>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct OnchainName {
    #[max_len(32)]
    pub name: String,
    pub bump: u8,
}

use anchor_lang::prelude::*;

use crate::policy::{Policy, TierInfo};

#[derive(Accounts)]
pub struct CreatePolicy<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(init,
        payer = payer,
        space = 8 + Policy::INIT_SPACE
    )]
    pub policy: Box<Account<'info, Policy>>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<CreatePolicy>, base_apr_x64: u128, tiers: Vec<TierInfo>) -> Result<()> {
    let policy = &mut ctx.accounts.policy;
    policy.update_authority = ctx.accounts.payer.key();
    policy.base_apr_x64 = base_apr_x64;
    policy.update_tiers(&tiers);
    Ok(())
}

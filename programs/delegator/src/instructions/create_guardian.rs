use anchor_lang::prelude::*;

use crate::guardian::Guardian;

#[derive(Accounts)]
pub struct CreateGuardian<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(init, payer = payer, space = 8 + Guardian::INIT_SPACE)]
    pub guardian: Box<Account<'info, Guardian>>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<CreateGuardian>) -> Result<()> {
    let guardian = &mut ctx.accounts.guardian;
    guardian.admin_authority = ctx.accounts.payer.key();
    Ok(())
}

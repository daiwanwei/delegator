use anchor_lang::prelude::*;
use anchor_spl::token_interface::{transfer, Mint, TokenAccount, TokenInterface, Transfer};

use crate::{guardian::Guardian, stake_pool::StakePool};

#[derive(Accounts)]
pub struct DepositReward<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut,
        has_one = reward_vault,
        has_one = reward_token_mint,
    )]
    pub stake_pool: Box<Account<'info, StakePool>>,

    #[account(mut,
        token::token_program = token_program,
        token::mint = reward_token_mint,
        token::authority = payer,
    )]
    pub payer_reward_token_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    pub reward_token_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(mut,
        token::token_program = token_program,
        token::mint = reward_token_mint,
    )]
    pub reward_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    pub token_program: Interface<'info, TokenInterface>,
}

pub fn handler(ctx: Context<DepositReward>, amount: u64) -> Result<()> {
    let transfer_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.payer_reward_token_ata.to_account_info(),
            to: ctx.accounts.reward_vault.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        },
    );
    transfer(transfer_ctx, amount)?;
    Ok(())
}

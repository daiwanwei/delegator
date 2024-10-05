use anchor_lang::prelude::*;
use anchor_spl::token_interface::{transfer, Mint, TokenAccount, TokenInterface, Transfer};

use crate::{
    error::ErrorCode,
    stake_pool::{self, StakePool},
    user_stake::UserStake,
};

#[derive(Accounts)]
pub struct Stake<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut,
        has_one = token_vault,
        has_one = token_mint,
    )]
    pub stake_pool: Box<Account<'info, StakePool>>,

    pub token_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(mut,
       constraint = user_stake.owner == payer.key(),
       constraint = user_stake.stake_pool == stake_pool.key(),
       constraint = user_stake.token_mint == token_mint.key(),
    )]
    pub user_stake: Box<Account<'info, UserStake>>,

    #[account(mut,
        token::token_program = token_program,
        token::mint = token_mint.key(),
        token::authority = payer,
    )]
    pub payer_token_ata: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(mut,
        token::token_program = token_program,
        token::mint = stake_pool.token_mint,
    )]
    pub token_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    pub token_program: Interface<'info, TokenInterface>,
}

pub fn handler(ctx: Context<Stake>, amount: u64) -> Result<()> {
    let stake_pool = &mut ctx.accounts.stake_pool;
    let mut staked_amount = stake_pool.total_staked_amount;
    staked_amount = staked_amount
        .checked_add(amount)
        .ok_or(ErrorCode::IntegerOverflow)?;

    if staked_amount > stake_pool.cap_stake_amount {
        return Err(ErrorCode::StakeAmountExceedsCap.into());
    }

    let transfer_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.payer_token_ata.to_account_info(),
            to: ctx.accounts.token_vault.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        },
    );
    transfer(transfer_ctx, amount)?;

    stake_pool.total_staked_amount = staked_amount;

    let user_stake = &mut ctx.accounts.user_stake;
    user_stake.staked_amount = user_stake
        .staked_amount
        .checked_add(amount)
        .ok_or(ErrorCode::IntegerOverflow)?;

    user_stake.last_stake_timestamp = Clock::get()?.unix_timestamp as u64;

    Ok(())
}

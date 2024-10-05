use core::time;

use anchor_lang::prelude::*;
use anchor_spl::token_interface::{transfer, Mint, TokenAccount, TokenInterface, Transfer};

use crate::{error::ErrorCode, stake_pool::StakePool, user_stake::UserStake};

#[derive(Accounts)]
pub struct Unstake<'info> {
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

pub fn handler(ctx: Context<Unstake>, amount: u64) -> Result<()> {
    let stake_pool = &mut ctx.accounts.stake_pool;

    if stake_pool.total_staked_amount < amount {
        return Err(ErrorCode::InvalidUnstakeAmount.into());
    }

    let user_stake = &mut ctx.accounts.user_stake;

    if user_stake.staked_amount < amount {
        return Err(ErrorCode::InvalidUnstakeAmount.into());
    }

    let episode = stake_pool.episode.to_be_bytes();

    let signer_seeds: [&[&[u8]]; 1] = [&[
        b"stake_pool".as_ref(),
        stake_pool.guardian.as_ref(),
        episode.as_ref(),
        &[stake_pool.bump],
    ]];

    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.token_vault.to_account_info(),
            to: ctx.accounts.payer_token_ata.to_account_info(),
            authority: stake_pool.to_account_info(),
        },
        &signer_seeds,
    );
    transfer(transfer_ctx, amount)?;

    stake_pool.total_staked_amount = stake_pool
        .total_staked_amount
        .checked_sub(amount)
        .ok_or(ErrorCode::InvalidUnstakeAmount)?;

    let user_stake = &mut ctx.accounts.user_stake;
    user_stake.staked_amount = user_stake
        .staked_amount
        .checked_sub(amount)
        .ok_or(ErrorCode::InvalidUnstakeAmount)?;

    let timestamp = Clock::get()?.unix_timestamp as u64;

    let interval = timestamp - user_stake.last_update_reward_timestamp;

    let reward = user_stake.base_apr_x64 as u64 * interval;
    user_stake.owed_reward += reward;

    user_stake.last_stake_timestamp = timestamp;
    user_stake.last_update_reward_timestamp = timestamp;

    Ok(())
}

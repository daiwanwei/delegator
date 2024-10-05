use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

use crate::{guardian::Guardian, policy::Policy, stake_pool::StakePool};

#[derive(Accounts)]
#[instruction(period: u8)]
pub struct CreateStakePool<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    pub guardian: Box<Account<'info, Guardian>>,

    pub policy: Box<Account<'info, Policy>>,

    #[account(init,
        seeds = [b"stake_pool".as_ref(), guardian.key().as_ref(),period.to_le_bytes().as_ref()],
        bump,
        payer = payer,
        space = 8 + StakePool::INIT_SPACE)]
    pub stake_pool: Box<Account<'info, StakePool>>,

    #[account(
        mint::token_program = token_program,
    )]
    pub token_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(
        mint::token_program = token_program_reward,
    )]
    pub reward_token_mint: Box<InterfaceAccount<'info, Mint>>,

    #[account(init,
        payer = payer,
        token::token_program = token_program,
        token::authority = stake_pool,
        token::mint = token_mint,
    )]
    pub token_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    #[account(init,
        payer = payer,
        token::token_program = token_program_reward,
        token::authority = stake_pool,
        token::mint = reward_token_mint,
    )]
    pub reward_vault: Box<InterfaceAccount<'info, TokenAccount>>,

    pub token_program: Interface<'info, TokenInterface>,

    pub token_program_reward: Interface<'info, TokenInterface>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<CreateStakePool>, episode: u8, cap_stake_amount: u64) -> Result<()> {
    let stake_pool = &mut ctx.accounts.stake_pool;
    stake_pool.guardian = ctx.accounts.guardian.key();
    stake_pool.policy = ctx.accounts.policy.key();
    stake_pool.episode = episode;
    stake_pool.token_mint = ctx.accounts.token_mint.key();
    stake_pool.token_vault = ctx.accounts.token_vault.key();
    stake_pool.reward_token_mint = ctx.accounts.reward_token_mint.key();
    stake_pool.reward_vault = ctx.accounts.reward_vault.key();
    stake_pool.total_staked_amount = 0;
    stake_pool.cap_stake_amount = cap_stake_amount;
    stake_pool.episode = episode;
    stake_pool.bump = ctx.bumps.stake_pool;
    Ok(())
}

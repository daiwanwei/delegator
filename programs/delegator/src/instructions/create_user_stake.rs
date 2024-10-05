use anchor_lang::prelude::*;

use crate::{policy::Policy, stake_pool::StakePool, user_stake::UserStake};

#[derive(Accounts)]
pub struct CreateUserStake<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(mut,
        has_one = policy,
    )]
    pub stake_pool: Box<Account<'info, StakePool>>,

    pub policy: Box<Account<'info, Policy>>,

    #[account(init,
        seeds = [b"user_stake".as_ref(), stake_pool.key().as_ref(), payer.key().as_ref()],
        bump,
        payer = payer,
        space = 8 + UserStake::INIT_SPACE
    )]
    pub user_stake: Box<Account<'info, UserStake>>,

    pub system_program: Program<'info, System>,

    pub rent: Sysvar<'info, Rent>,
}

pub fn handler(ctx: Context<CreateUserStake>, tier: u8) -> Result<()> {
    let user_stake = &mut ctx.accounts.user_stake;
    user_stake.stake_pool = ctx.accounts.stake_pool.key();
    user_stake.policy = ctx.accounts.stake_pool.policy;
    user_stake.owner = ctx.accounts.payer.key();
    user_stake.token_mint = ctx.accounts.stake_pool.token_mint;
    user_stake.staked_amount = 0;

    user_stake.owed_reward = 0;
    user_stake.base_apr_x64 = ctx.accounts.policy.base_apr_x64;
    user_stake.tier = ctx.accounts.policy.tiers[tier as usize].tier;
    user_stake.tier_lock_up_epoch = ctx.accounts.policy.tiers[tier as usize].lock_up_epoch;
    user_stake.tier_multiplier = ctx.accounts.policy.tiers[tier as usize].multiplier;

    let timestamp = Clock::get()?.unix_timestamp as u64;
    user_stake.last_stake_timestamp = timestamp;
    user_stake.last_update_reward_timestamp = timestamp;

    Ok(())
}

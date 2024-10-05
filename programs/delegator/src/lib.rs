pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
use state::policy::TierInfo;
pub use state::*;

declare_id!("DTLVZKAFzv9JFYh6vXyBQdCoFcFspqMh6yCmmz1FyKCD");

#[program]
pub mod delegator {
    use policy::TierInfo;

    use super::*;

    pub fn create_guardian(ctx: Context<CreateGuardian>) -> Result<()> {
        create_guardian::handler(ctx)
    }

    pub fn create_stake_pool(
        ctx: Context<CreateStakePool>,
        episode: u8,
        cap_stake_amount: u64,
    ) -> Result<()> {
        create_stake_pool::handler(ctx, episode, cap_stake_amount)
    }

    pub fn create_policy(
        ctx: Context<CreatePolicy>,
        base_apr_x64: u128,
        tiers: Vec<TierInfo>,
    ) -> Result<()> {
        create_policy::handler(ctx, base_apr_x64, tiers)
    }

    pub fn deposit_reward(ctx: Context<DepositReward>, amount: u64) -> Result<()> {
        deposit_reward::handler(ctx, amount)
    }

    pub fn create_user_stake(ctx: Context<CreateUserStake>, tier: u8) -> Result<()> {
        create_user_stake::handler(ctx, tier)
    }

    pub fn stake(ctx: Context<Stake>, amount: u64) -> Result<()> {
        stake::handler(ctx, amount)
    }

    pub fn unstake(ctx: Context<Unstake>, amount: u64) -> Result<()> {
        unstake::handler(ctx, amount)
    }
}

use anchor_lang::prelude::*;

#[account]
#[derive(Default, Debug, InitSpace)]
pub struct UserStake {
    pub stake_pool: Pubkey,

    pub policy: Pubkey,

    pub owner: Pubkey,

    pub token_mint: Pubkey,

    pub staked_amount: u64,
    pub last_stake_timestamp: u64,

    pub base_apr_x64: u128,
    pub tier: u8,
    pub tier_lock_up_epoch: u8,
    pub tier_multiplier: u8,

    pub owed_reward: u64,
    pub last_update_reward_timestamp: u64,
}

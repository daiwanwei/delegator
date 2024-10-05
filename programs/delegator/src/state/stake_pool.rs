use anchor_lang::prelude::*;

#[account]
#[derive(Default, Debug, InitSpace)]
pub struct StakePool {
    pub guardian: Pubkey,

    pub policy: Pubkey,

    pub token_mint: Pubkey,

    pub token_vault: Pubkey,

    pub reward_token_mint: Pubkey,

    pub reward_vault: Pubkey,

    pub total_staked_amount: u64,

    pub cap_stake_amount: u64,

    pub episode: u8,

    pub bump: u8,
}

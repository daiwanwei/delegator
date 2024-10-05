use anchor_lang::prelude::*;

const MAX_TIERS: usize = 8;

#[account]
#[derive(Debug, InitSpace)]
pub struct Policy {
    pub update_authority: Pubkey,

    pub base_apr_x64: u128,

    pub tiers: [TierInfo; MAX_TIERS],
}

impl Default for Policy {
    fn default() -> Self {
        Self {
            update_authority: Pubkey::default(),
            base_apr_x64: 0,
            tiers: [TierInfo::default(); MAX_TIERS],
        }
    }
}

impl Policy {
    pub fn update_tiers(&mut self, tier_info: &[TierInfo]) {
        let len = std::cmp::min(tier_info.len(), MAX_TIERS);
        for i in 0..len {
            self.tiers[i] = tier_info[i];
        }
    }
}

#[derive(AnchorSerialize, AnchorDeserialize, Copy, Clone, Debug, Default, PartialEq, InitSpace)]
pub struct TierInfo {
    pub tier: u8,
    pub lock_up_epoch: u8,
    pub multiplier: u8,
}

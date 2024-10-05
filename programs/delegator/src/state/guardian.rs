use anchor_lang::prelude::*;

#[account]
#[derive(Default, Debug, InitSpace)]
pub struct Guardian {
    pub admin_authority: Pubkey,
}

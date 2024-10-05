use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Custom error message")]
    CustomError,

    #[msg("Integer overflow")]
    IntegerOverflow,

    #[msg("Stake amount exceeds cap")]
    StakeAmountExceedsCap,

    #[msg("Invalid unstake amount")]
    InvalidUnstakeAmount,
}

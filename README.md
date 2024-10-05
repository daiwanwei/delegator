<h1 align="center">Delegator Program<h1>

<details>
<summary>Table of Contents</summary>

- [Introduction](#introduction)
- [Instructions](#instructions)
- [Accounts](#account)
- [Building And Testing](#building-and-testing)

</details>

## Introduction
The Delegator Program is implemented in Rust and leverages the Anchor framework for Solana.

## Instructions
- For Guardian
    - `create_guardian`
    - `update_guardian`
    - `create_stake_pool`
    - `create_policy`
    - `deposit_reward`
    - `withdraw_reward`
    - `withdraw_from_treasury`
- For User
    - `initialize_user_stake`
    - `stake`
    - `request_unstake`
    - `claim_reward`
- For backend Bot
    - `unstake`

## Accounts
- Guardian
    - `admin_authority` : `Pubkey`
    - `treasury_authority` : `Pubkey`
- UserStake
    - `policy` : `Pubkey`
    - `owner` : `Pubkey`
    - `token_mint` : `Pubkey`
    - `stake_pool` : `Pubkey`
    - `staked_amount` : `u64`
    - `created_timestamp` : `u64`
- StakePool
    - `guardian` : `Pubkey`
    - `token_mint` : `Pubkey`
    - `reward_token_mint` : `Pubkey`
    - `token_vault` : `Pubkey`
    - `reward_vault` : `Pubkey`
    - `treasury_vault` : `Pubkey`
    - `policy` : `Pubkey`
    - `treasury_authority` : `Pubkey`
    - `policy_authority` : `Pubkey`
    - `total_staked_amount` : `u64`
    - `cap_stake_amount` : `u64`
    - `episode` : `u8`
- Policy
    - `update_authority` : `Pubkey`
    - `tiers` : `[TierInfo, 8]`

## Building and Testing
To build the Delegator Program, navigate to the delegator directory and run:

```shell
$ anchor build
```

To test the program, navigate to the root directory and run:

```shell
$ anchor test
```
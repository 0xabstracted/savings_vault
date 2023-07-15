use anchor_lang::prelude::*;

pub const SEED_INTEREST_DEPOSITOR_MANAGER: &[u8] = b"interest_depositor_manager";
pub const SEED_INTEREST_DEPOSITOR_TREASURY: &[u8] = b"interest_depositor_treasury";

pub const WHITELIST_SIZE: usize = 10;

pub const SIZE_OF_INTEREST_DEPOSITOR_MANAGER: usize = 1 + 32 + 8 + 32 + 1 + WHITELIST_SIZE * 32;

#[account]
#[derive(Default, Debug)]
pub struct InterestDepositorManager {
    pub bump: u8,
    pub authority: Pubkey,
    pub mint: Pubkey,
    pub total_liquidity_for_interest: u64,
    pub interest_depositor_treasury: Pubkey,
    pub interest_depositor_treasury_bump: u8,
    pub whitelist: Vec<Pubkey>,
}

#[event]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CreateInterestDepositorManagerEvent {
    pub mint: Pubkey,
    pub whitelist: Vec<Pubkey>,
}

#[event]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AddWhitelistEvent {
    pub mint: Pubkey,
    pub whitelist_entry: Pubkey,
}

#[event]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RemoveWhitelistEvent {
    pub mint: Pubkey,
    pub whitelist_entry: Pubkey,
}

#[event]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DepositLiquidityForInterestEvent {
    pub mint: Pubkey,
    pub depositor: Pubkey,
    pub number_of_tokens_deposited: u64,
}

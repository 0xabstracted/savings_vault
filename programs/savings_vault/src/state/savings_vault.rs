use anchor_lang::prelude::*;

pub const SEED_SAVINGS_VAULT: &[u8] = b"savings_vault";
pub const SEED_SAVINGS_VAULT_TREASURY: &[u8] = b"savings_vault-treasury";

pub const SIZE_OF_SAVINGS_VAULT: usize = 1 + 32 + 32 + 8 + 8 + 8 + 32 + 1;

pub const SECONDS_PER_MONTH: i64 = 30 * 24 * 60 * 60;
pub const INTEREST_RATE_PER_MONTH: u64 = 1;

#[account]
#[derive(Default, Debug)]
pub struct SavingsVault {
    pub bump: u8,
    pub wallet: Pubkey,
    pub mint: Pubkey,
    pub created_ts: i64,
    pub last_accrued_ts: i64,
    pub starting_balance: u64,
    pub current_balance: u64,
    pub interest_depositor_manager: Pubkey,
    pub savings_vault_treasury: Pubkey,
    pub savings_vault_treasury_bump: u8,
    pub amount_withdrawn: u64,
    pub last_withdrawn_ts: i64,
}

#[event]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CreateSavingsVaultEvent {
    pub wallet: Pubkey,
    pub mint: Pubkey,
    pub created_ts: i64,
    pub current_balance: u64,
}

#[event]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct DepositSavingsVaultEvent {
    pub wallet: Pubkey,
    pub mint: Pubkey,
    pub number_of_additional_tokens_deposited: u64,
    pub current_balance: u64,
}

#[event]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct WithdrawSavingsVaultEvent {
    pub wallet: Pubkey,
    pub mint: Pubkey,
    pub number_of_tokens_withdrawn: u64,
    pub current_balance: u64,
}

#[event]
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AccrueInterestEvent {
    pub wallet: Pubkey,
    pub mint: Pubkey,
    pub number_of_tokens_accrued: u64,
    pub current_balance: u64,
}
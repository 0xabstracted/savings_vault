pub mod state;
pub mod utils;
pub mod error;
pub mod instructions;

pub use anchor_lang::prelude::*;
pub use instructions::*;

declare_id!("7UGWo3y9HkLof88nHW3LGYX5hVts9WAMMmEeYYLK8dKB");

#[program]
pub mod savings_vault {
    use super::*;

    // InterestDepositorManager
    pub fn create_interest_depositor_manager(ctx: Context<CreateInterestDepositorManager>, interest_depositor_treasury_bump: u8) -> Result<()> {
        create_interest_depositor_manager::create_interest_depositor_manager(ctx, interest_depositor_treasury_bump)
    }
    pub fn add_whitelist(ctx: Context<AddWhitelist>, whitelist_entry: Pubkey,) -> Result<()> {
        add_whitelist::add_whitelist(ctx, whitelist_entry)
    }
    pub fn remove_whitelist(ctx: Context<RemoveWhitelist>, whitelist_entry: Pubkey,) -> Result<()> {
        remove_whitelist::remove_whitelist(ctx, whitelist_entry)
    }
    pub fn deposit_liquidity_for_interest(ctx: Context<DepositLiquidityForInterest>, number_of_tokens_deposited: u64,) -> Result<()> {
        deposit_liquidity_for_interest::deposit_liquidity_for_interest(ctx, number_of_tokens_deposited)
    }

    // SavingsVault
    pub fn create_savings_vault(ctx: Context<CreateSavingsVault>,number_of_tokens_deposited: u64, savings_vault_treasury_bump: u8,) -> Result<()> {
        create_savings_vault::create_savings_vault(ctx, number_of_tokens_deposited, savings_vault_treasury_bump)
    }
    pub fn deposit_savings_vault(ctx: Context<DepositSavingsVault>, number_of_additional_tokens_deposited: u64,) -> Result<()> {
        deposit_savings_vault::deposit_savings_vault(ctx, number_of_additional_tokens_deposited)
    }
    pub fn accrue_interest(ctx: Context<AccrueInterest>) -> Result<()> {
        accrue_interest::accrue_interest(ctx)
    }
    pub fn withdraw_savings_vault(ctx: Context<WithdrawSavingsVault>, amount_to_withdraw: u64,) -> Result<()> {
        withdraw_savings_vault::withdraw_savings_vault(ctx, amount_to_withdraw)
    }
    
}

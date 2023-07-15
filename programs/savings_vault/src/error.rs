use anchor_lang::prelude::*;

#[error_code]
pub enum SavingsVaultError {
    #[msg("The bump seed provided does not match the bump seed generated from the account seeds.")]
    BumpSeedNotInHashMap,
    #[msg("Pubkey mismatch.")]
    PublicKeyMismatch,
    #[msg("The account is not initialized.")]
    UninitializedAccount,
    #[msg("Incorrect owner provided.")]
    IncorrectOwner,
    #[msg("The derived key is invalid.")]
    DerivedKeyInvalid,
    #[msg("Numerical overflow.")]
    NumericalOverflow,   
    #[msg("Invalid associated token account.")]
    InvalidAssociatedTokenAccount,
    #[msg("Invalid deposit amount.")]
    InvalidDepositAmount,
    #[msg("Interest Depositor Whitelist full.")]
    WhitelistFull,
    #[msg("Whitelist entry not found.")]
    WhitelistEntryNotFound,
    #[msg("Cranked too recently.")]
    CrankedTooRecently,
    #[msg("Not enough interest to accrue.")]
    NotEnoughInterestToAccrue,
    #[msg("Insufficient funds to withdraw from savings vault.")]
    InsufficientFundsToWithdrawSavingsVault,
}
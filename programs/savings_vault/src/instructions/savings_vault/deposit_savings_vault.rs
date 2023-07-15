use {
    anchor_lang::prelude::*,
    anchor_spl::token::{self,TokenAccount, Mint},
    solana_program::program::invoke_signed,
    crate::{
        state::{
            SavingsVault, 
            InterestDepositorManager,
            DepositSavingsVaultEvent,
            SEED_SAVINGS_VAULT, 
        },
    },  
};

#[derive(Accounts)]
#[instruction(number_of_additional_tokens_deposited: u64)]
pub struct DepositSavingsVault<'info> {
    /// Authority that creates SavingsVault
    pub wallet: Signer<'info>,

    #[account(
        mut, 
        constraint = wallet_token_account.owner == wallet.key(),
        constraint = wallet_token_account.mint == mint.key(),
        constraint = wallet_token_account.amount >= number_of_additional_tokens_deposited,
    )]
    pub wallet_token_account: Box<Account<'info, TokenAccount>>,
    
    #[account(
        mut,
        seeds = [
            SEED_SAVINGS_VAULT,
            wallet.key().as_ref(), 
            mint.key().as_ref(),
        ],
        bump = savings_vault.bump,
    )]
    pub savings_vault: Box<Account<'info, SavingsVault>>,

    #[account(mut, address = savings_vault.savings_vault_treasury,)]
    pub savings_vault_treasury: Box<Account<'info, TokenAccount>>,

    #[account(address = savings_vault.interest_depositor_manager)]
    pub interest_depositor_manager: Box<Account<'info, InterestDepositorManager>>,
    
    #[account(address = interest_depositor_manager.mint)]
    pub mint: Box<Account<'info, Mint>>,

    pub token_program: Program<'info, token::Token>,
}

pub fn deposit_savings_vault(
    ctx: Context<DepositSavingsVault>,
    number_of_additional_tokens_deposited: u64,
) -> Result<()> {
    let wallet = &ctx.accounts.wallet;
    let savings_vault = &mut ctx.accounts.savings_vault;
    let wallet_token_account = &mut ctx.accounts.wallet_token_account;
    let savings_vault_treasury = &mut ctx.accounts.savings_vault_treasury;
    let mint = &ctx.accounts.mint;
    let token_program = &ctx.accounts.token_program;

    invoke_signed(
        &spl_token::instruction::transfer(
            token_program.key,
            &wallet_token_account.key(),
            &savings_vault_treasury.key(),
            &wallet.key(),
            &[],
            number_of_additional_tokens_deposited,
        )?,
        &[
            wallet_token_account.to_account_info(),
            savings_vault_treasury.to_account_info(),
            token_program.to_account_info(),
            wallet.to_account_info(),
        ],
        &[],
    )?;

    savings_vault.current_balance += number_of_additional_tokens_deposited;

    emit!(DepositSavingsVaultEvent {
        wallet: wallet.key(), 
        mint: mint.key(), 
        number_of_additional_tokens_deposited, 
        current_balance: savings_vault.current_balance, 
    });

    Ok(())
}
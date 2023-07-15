use {
    anchor_lang::prelude::*,
    anchor_spl::token::{self,TokenAccount, Mint},
    crate::{
        error::SavingsVaultError,
        state::{
            SavingsVault, 
            SEED_SAVINGS_VAULT, 
            SEED_SAVINGS_VAULT_TREASURY,
        },
    },  
};


#[derive(Accounts)]
pub struct WithdrawSavingsVault<'info> {
    #[account(address = savings_vault.wallet)]
    pub wallet: Signer<'info>,
    
    #[account(mut, 
        constraint = wallet_token_account.owner == wallet.key(),
        constraint = wallet_token_account.mint == mint.key(),
    )]
    pub wallet_token_account: Account<'info, TokenAccount>,

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

    #[account(address = savings_vault.mint)]
    pub mint: Box<Account<'info, Mint>>,

    pub token_program: Program<'info, token::Token>,
    pub clock: Sysvar<'info, Clock>,
}

pub fn withdraw_savings_vault(
    ctx: Context<WithdrawSavingsVault>,
    amount_to_withdraw: u64,
) -> Result<()> {
    let savings_vault = &mut ctx.accounts.savings_vault;
    let wallet_token_account = &mut ctx.accounts.wallet_token_account;
    let savings_vault_treasury = &mut ctx.accounts.savings_vault_treasury;
    let token_program = &ctx.accounts.token_program;
    let clock = &ctx.accounts.clock;

    if savings_vault.current_balance < amount_to_withdraw {
        return Err(SavingsVaultError::InsufficientFundsToWithdrawSavingsVault.into());
    }

    let savings_vault_key = savings_vault.key();
    
    let savings_vault_treasury_seeds = [
        SEED_SAVINGS_VAULT_TREASURY,
        savings_vault_key.as_ref(),
        &[savings_vault.savings_vault_treasury_bump],
    ];

    token::transfer(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            token::Transfer {
                from: savings_vault_treasury.to_account_info(),
                to: wallet_token_account.to_account_info(),
                authority: savings_vault.to_account_info(),
            },
            &[&savings_vault_treasury_seeds],
        ),
        amount_to_withdraw
    )?;

    savings_vault.current_balance -= amount_to_withdraw;
    savings_vault.last_withdrawn_ts = clock.unix_timestamp;
    savings_vault.amount_withdrawn += amount_to_withdraw;

    Ok(())
}

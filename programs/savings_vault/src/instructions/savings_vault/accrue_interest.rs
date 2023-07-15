use {
    anchor_lang::prelude::*,
    anchor_spl::token::{self, Mint, TokenAccount},
    crate::{
        error::SavingsVaultError,
        state::{
            SavingsVault,
            InterestDepositorManager,
            AccrueInterestEvent, 
            SEED_SAVINGS_VAULT, 
            SEED_SAVINGS_VAULT_TREASURY,
            SEED_INTEREST_DEPOSITOR_TREASURY,
            INTEREST_RATE_PER_MONTH,
            SECONDS_PER_MONTH,
        }
    },
};

#[derive(Accounts)]
pub struct AccrueInterest<'info> {

    /// Authority that cranks SavingsVault for interest
    /// CHECK: Not dangerous. 
    pub cranker: AccountInfo<'info>,

    /// CHECK: Not dangerous. Checked in constraint.
    #[account(address = savings_vault.wallet)]
    pub wallet: AccountInfo<'info>,

    #[account(
        mut,
        seeds = [
            SEED_SAVINGS_VAULT, 
            wallet.key().as_ref(),
            mint.key().as_ref(),    
        ],
        bump = savings_vault.bump,
        has_one = savings_vault_treasury,
    )]
    
    pub savings_vault: Box<Account<'info, SavingsVault>>,
    
    #[account(
        mut, 
        seeds = [
            SEED_SAVINGS_VAULT_TREASURY, 
            savings_vault.key().as_ref()
        ],
        bump = savings_vault.savings_vault_treasury_bump
    )]
    pub savings_vault_treasury: Box<Account<'info,TokenAccount>>,

    #[account(address = savings_vault.interest_depositor_manager)]
    pub interest_depositor_manager: Box<Account<'info, InterestDepositorManager>>,

    #[account(address = interest_depositor_manager.interest_depositor_treasury)]
    pub interest_depositor_treasury: Box<Account<'info, TokenAccount>>,
    
    #[account(address = savings_vault.mint)]
    pub mint: Box<Account<'info, Mint>>,

    pub token_program: Program<'info, token::Token>,
    pub clock: Sysvar<'info, Clock>,
}

pub fn accrue_interest(
    ctx: Context<AccrueInterest>,
) -> Result<()> {
    let wallet = &ctx.accounts.wallet;
    let savings_vault = &mut ctx.accounts.savings_vault;
    let savings_vault_treasury = &mut ctx.accounts.savings_vault_treasury;
    let interest_depositor_manager = &ctx.accounts.interest_depositor_manager;
    let interest_depositor_treasury = &mut ctx.accounts.interest_depositor_treasury;
    let mint = &ctx.accounts.mint;
    let token_program = &ctx.accounts.token_program;
    let clock = &ctx.accounts.clock;

    if clock.unix_timestamp < savings_vault.last_accrued_ts + SECONDS_PER_MONTH {
        return Err(SavingsVaultError::CrankedTooRecently.into());
    }

    let interest_to_accrue = savings_vault.current_balance * INTEREST_RATE_PER_MONTH / 100;

    if interest_depositor_treasury.amount < interest_to_accrue {
        return Err(SavingsVaultError::NotEnoughInterestToAccrue.into());
    }

    let interest_depositor_manager_key = interest_depositor_manager.key();

    let interest_depositor_treasury_seeds = [
        SEED_INTEREST_DEPOSITOR_TREASURY,
        interest_depositor_manager_key.as_ref(),
        &[interest_depositor_manager.interest_depositor_treasury_bump],
    ];

    token::transfer(
        CpiContext::new_with_signer(
            token_program.to_account_info(),
            token::Transfer {
                from: interest_depositor_treasury.to_account_info(),
                to: savings_vault_treasury.to_account_info(),
                authority: interest_depositor_manager.to_account_info(),
            },
            &[&interest_depositor_treasury_seeds],
        ),
        interest_to_accrue
    )?;

    savings_vault.current_balance += interest_to_accrue;
    savings_vault.last_accrued_ts = clock.unix_timestamp;

    emit!(AccrueInterestEvent {
        wallet:wallet.key(),
        mint:mint.key(),
        number_of_tokens_accrued: interest_to_accrue,
        current_balance:savings_vault.current_balance, 
    });

    Ok(())
}
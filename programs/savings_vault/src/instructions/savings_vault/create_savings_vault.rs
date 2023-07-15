use {
    anchor_lang::prelude::*,
    anchor_spl::token::{self,TokenAccount, Mint},
    solana_program::program::invoke_signed,
    crate::{
        utils::create_program_token_account_if_not_present,
        error::SavingsVaultError,
        state::{
            SavingsVault, 
            CreateSavingsVaultEvent,
            InterestDepositorManager,
            SEED_SAVINGS_VAULT, 
            SEED_SAVINGS_VAULT_TREASURY,
            SEED_INTEREST_DEPOSITOR_MANAGER, 
            SIZE_OF_SAVINGS_VAULT, 
        },
    },  
};

#[derive(Accounts)]
#[instruction(number_of_tokens_deposited: u64)]
pub struct CreateSavingsVault<'info> {
    /// Authority that creates SavingsVault
    #[account(mut)]
    pub wallet: Signer<'info>,

    #[account(mut, 
        constraint = wallet_token_account.owner == wallet.key(),
        constraint = wallet_token_account.mint == mint.key(),
        constraint = wallet_token_account.amount >= number_of_tokens_deposited,
    )]
    pub wallet_token_account: Account<'info, TokenAccount>,
    
    /// SavingsVault instance
    #[account(
        init,
        payer = wallet,
        seeds = [
            SEED_SAVINGS_VAULT,
            wallet.key().as_ref(), 
            mint.key().as_ref(),
        ],
        bump,
        space = 8 + SIZE_OF_SAVINGS_VAULT,
    )]
    pub savings_vault: Box<Account<'info, SavingsVault>>,

    /// CHECK: Not dangerous. Account seeds checked in constraint.
    #[account(
        mut,
        seeds = [
            SEED_SAVINGS_VAULT_TREASURY,
            savings_vault.key().as_ref(),
        ],
        bump,
    )]
    pub savings_vault_treasury: UncheckedAccount<'info>,

    #[account(
        seeds = [
            SEED_INTEREST_DEPOSITOR_MANAGER,
            mint.key().as_ref(),
        ],
        bump,
    )]
    pub interest_depositor_manager: Box<Account<'info, InterestDepositorManager>>,
    
    pub mint: Box<Account<'info, Mint>>,

    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
    pub clock: Sysvar<'info, Clock>,
}

pub fn create_savings_vault(
    ctx: Context<CreateSavingsVault>,
    number_of_tokens_deposited: u64,
    savings_vault_treasury_bump: u8,
) -> Result<()>{

    if number_of_tokens_deposited == 0 {
        return Err(SavingsVaultError::InvalidDepositAmount.into());
    }

    let wallet = &ctx.accounts.wallet;
    let wallet_token_account = &mut ctx.accounts.wallet_token_account;
    let savings_vault = &mut ctx.accounts.savings_vault;
    let savings_vault_treasury = &mut ctx.accounts.savings_vault_treasury;
    let interest_depositor_manager = &ctx.accounts.interest_depositor_manager;
    let mint = &ctx.accounts.mint;
    let rent = &ctx.accounts.rent;
    let system_program = &ctx.accounts.system_program;
    let token_program = &ctx.accounts.token_program;
    let clock = &ctx.accounts.clock;

    msg!("savings_vault_treasury_bump: {}", savings_vault_treasury_bump);
    let bump_from_account = *ctx.bumps.get("savings_vault_treasury").ok_or(SavingsVaultError::BumpSeedNotInHashMap)?;
    msg!("bump_from_account: {}", bump_from_account);
    if savings_vault_treasury_bump != bump_from_account {
        return Err(SavingsVaultError::BumpSeedNotInHashMap.into());
    }


    let savings_vault_key = savings_vault.key();

    let savings_vault_treasury_seeds = [
        SEED_SAVINGS_VAULT_TREASURY,
        savings_vault_key.as_ref(),
        &[savings_vault_treasury_bump],
    ];

    let is_native = mint.key() == spl_token::native_mint::id();

    create_program_token_account_if_not_present(
        savings_vault_treasury,
        system_program,
        wallet,
        token_program,
        mint,
        &savings_vault.to_account_info(),
        rent,
        &savings_vault_treasury_seeds,
        &[],
        is_native,
    )?;

    //transfer tokens from wallet to savings_vault_treasury
    invoke_signed(
        &spl_token::instruction::transfer(
            token_program.key,
            &wallet_token_account.key(),
            &savings_vault_treasury.key(),
            &wallet.key(),
            &[],
            number_of_tokens_deposited,
        )?,
        &[
            wallet_token_account.to_account_info(),
            savings_vault_treasury.to_account_info(),
            token_program.to_account_info(),
            wallet.to_account_info(),
        ],
        &[],
    )?;


    msg!("Initializing savings vault account...");

    let bump = *ctx.bumps.get("savings_vault").unwrap();
    savings_vault.bump = bump;
    savings_vault.wallet = wallet.key();
    savings_vault.mint = mint.key();
    savings_vault.created_ts = clock.unix_timestamp;
    savings_vault.last_accrued_ts = clock.unix_timestamp;
    savings_vault.starting_balance = number_of_tokens_deposited;
    savings_vault.current_balance = number_of_tokens_deposited;      
    savings_vault.interest_depositor_manager = interest_depositor_manager.key();
    savings_vault.savings_vault_treasury = savings_vault_treasury.key();
    savings_vault.savings_vault_treasury_bump = savings_vault_treasury_bump;
    savings_vault.amount_withdrawn = 0;
    savings_vault.last_withdrawn_ts = 0;
    
    emit!(CreateSavingsVaultEvent{
        wallet: wallet.key(),
        mint: mint.key(),
        created_ts: savings_vault.created_ts,
        current_balance: savings_vault.current_balance,
    });
    
    Ok(())
}
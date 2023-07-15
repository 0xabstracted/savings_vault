use {
    anchor_lang::prelude::*,
    anchor_spl::token::{self, Mint, TokenAccount},
    crate::{
        state::{
            InterestDepositorManager, 
            DepositLiquidityForInterestEvent,
            SEED_INTEREST_DEPOSITOR_MANAGER, 
        }, 
    },
};


#[derive(Accounts)]
#[instruction(number_of_tokens_deposited: u64)]
pub struct DepositLiquidityForInterest<'info> {
    // check if the depositor is whitelisted
    #[account(constraint = interest_depositor_manager.whitelist.contains(&depositor.key()))]
    pub depositor: Signer<'info>,
    
    #[account(
        mut,
        constraint = depositor_token_account.owner == depositor.key(),
        constraint = depositor_token_account.mint == mint.key(),
        constraint = depositor_token_account.amount >= number_of_tokens_deposited,
    )]
    pub depositor_token_account: Box<Account<'info, TokenAccount>>,
    
    #[account(
        mut,
        seeds = [
            SEED_INTEREST_DEPOSITOR_MANAGER,
            interest_depositor_manager.mint.key().as_ref(),
        ],
        bump = interest_depositor_manager.bump,
    )]
    pub interest_depositor_manager: Box<Account<'info, InterestDepositorManager>>,
    
    #[account(
        mut,
        constraint = interest_depositor_manager.interest_depositor_treasury == interest_depositor_treasury.key(),
    )]
    pub interest_depositor_treasury: Box<Account<'info, TokenAccount>>,
    
    #[account(address = interest_depositor_manager.mint)]
    pub mint: Box<Account<'info, Mint>>,

    pub token_program: Program<'info, token::Token>,
}

pub fn deposit_liquidity_for_interest(
    ctx: Context<DepositLiquidityForInterest>,
    number_of_tokens_deposited: u64,
) -> Result<()> {
    let interest_depositor_manager = &mut ctx.accounts.interest_depositor_manager;
    let depositor_token_account = &mut ctx.accounts.depositor_token_account;
    let interest_depositor_treasury = &mut ctx.accounts.interest_depositor_treasury;
    let mint = &ctx.accounts.mint;
    let depositor = &ctx.accounts.depositor;
    let token_program = &ctx.accounts.token_program;

    // transfer tokens from depositor to interest_depositor_treasury
    let cpi_accounts = token::Transfer {
        from: depositor_token_account.to_account_info(),
        to: interest_depositor_treasury.to_account_info(),
        authority: depositor.to_account_info(),
    };
    let cpi_program = token_program.to_account_info().clone();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);
    token::transfer(cpi_ctx, number_of_tokens_deposited)?;

    // update interest_depositor_manager
    interest_depositor_manager.total_liquidity_for_interest += number_of_tokens_deposited;

    // emit event
    let event = DepositLiquidityForInterestEvent {
        mint: mint.key(),
        depositor: depositor.key(),
        number_of_tokens_deposited,
    };
    emit!(event);

    Ok(())
}
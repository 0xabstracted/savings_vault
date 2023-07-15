use {
    anchor_lang::prelude::*,
    anchor_spl::token::{self, Mint},
    crate::{
        error::SavingsVaultError, 
        state::{
            InterestDepositorManager, 
            CreateInterestDepositorManagerEvent,
            SEED_INTEREST_DEPOSITOR_MANAGER, 
            SEED_INTEREST_DEPOSITOR_TREASURY, 
            SIZE_OF_INTEREST_DEPOSITOR_MANAGER, 
        }, 
        utils::create_program_token_account_if_not_present,
    },
};


#[derive(Accounts)]
pub struct CreateInterestDepositorManager<'info> {
    /// Authority that creates InterestDepositorManager
    #[account(mut)]
    pub authority: Signer<'info>,
    
    /// InterestDepositorManager instance
    #[account(
        init,
        payer = authority,
        seeds = [
            SEED_INTEREST_DEPOSITOR_MANAGER,
            mint.key().as_ref(),
        ],
        bump,
        space = 8 + SIZE_OF_INTEREST_DEPOSITOR_MANAGER,
    )]
    pub interest_depositor_manager: Box<Account<'info, InterestDepositorManager>>,

    /// CHECK: Not dangerous. Account seeds checked in constraint.
    #[account(
        mut,
        seeds = [
            SEED_INTEREST_DEPOSITOR_TREASURY,
            interest_depositor_manager.key  ().as_ref(),
        ],
        bump,
    )]
    pub interest_depositor_treasury: UncheckedAccount<'info>,
    
    pub mint: Box<Account<'info, Mint>>,
    
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, token::Token>,
}

pub fn create_interest_depositor_manager(
    ctx: Context<CreateInterestDepositorManager>,
    interest_depositor_treasury_bump: u8,
) -> Result<()> {

    let authority = &ctx.accounts.authority;
    let interest_depositor_manager = &mut ctx.accounts.interest_depositor_manager;
    let interest_depositor_treasury = &mut ctx.accounts.interest_depositor_treasury;
    let mint = &ctx.accounts.mint;
    let rent = &ctx.accounts.rent;
    let system_program = &ctx.accounts.system_program;
    let token_program = &ctx.accounts.token_program;

    msg!("interest_depositor_treasury_bump: {}", interest_depositor_treasury_bump);
    let bump_from_account = *ctx.bumps.get("interest_depositor_treasury").ok_or(SavingsVaultError::BumpSeedNotInHashMap)?;
    msg!("bump_from_account: {}", bump_from_account);
    if interest_depositor_treasury_bump != bump_from_account {
        return Err(SavingsVaultError::BumpSeedNotInHashMap.into());
    }

    let interest_depositor_manager_key = interest_depositor_manager.key();

    let interest_depositor_treasury_seeds = [
        SEED_INTEREST_DEPOSITOR_TREASURY,
        interest_depositor_manager_key.as_ref(),
        &[interest_depositor_treasury_bump],
    ];

    let is_native = mint.key() == spl_token::native_mint::id();

    create_program_token_account_if_not_present(
        interest_depositor_treasury,
        system_program,
        authority,
        token_program,
        mint,
        &interest_depositor_manager.to_account_info(),
        rent,
        &interest_depositor_treasury_seeds,
        &[],
        is_native,
    )?;

    let bump = *ctx.bumps.get("interest_depositor_manager").unwrap();
    interest_depositor_manager.bump = bump;
    interest_depositor_manager.authority = authority.key();
    interest_depositor_manager.mint = mint.key();
    interest_depositor_manager.total_liquidity_for_interest = 0;
    interest_depositor_manager.interest_depositor_treasury = interest_depositor_treasury.key();
    interest_depositor_manager.interest_depositor_treasury_bump = interest_depositor_treasury_bump;
    interest_depositor_manager.whitelist = vec![];

    emit!(CreateInterestDepositorManagerEvent {
        mint: mint.key(),
        whitelist: interest_depositor_manager.whitelist.clone(),
    });
    // let interest_depositor_mint_key = mint.key();
    // let interest_depositor_seeds = &[
    //     b"interest_depositor".as_ref(), 
    //     interest_depositor_mint_key.as_ref(),
    //     &interest_depositor_manager.bump
    // ];
    // let interest_depositor_signer = &[&interest_depositor_seeds[..]];

    // let cpi_accounts_token_interest_depositor = Transfer {
    //     from: authority_token_account.to_account_info(),
    //     to: interest_depositor_treasury.to_account_info(),
    //     authority: interest_depositor_manager.to_account_info(),
    // };
    // let cpi_program = token_program.to_account_info();
    // let cpi_context_token_interest_depositor = CpiContext::new(cpi_program, cpi_accounts_token_interest_depositor);
    // token::transfer(cpi_context_token_interest_depositor, token_liquidity_provided)?;

    Ok(())
}
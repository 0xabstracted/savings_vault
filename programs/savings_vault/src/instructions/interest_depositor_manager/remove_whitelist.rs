use {
    anchor_lang::prelude::*,
    anchor_spl::token::Mint,
    crate::{
        error::SavingsVaultError, 
        state::{
            InterestDepositorManager, 
            RemoveWhitelistEvent,
            SEED_INTEREST_DEPOSITOR_MANAGER, 
        }, 
    },
};


#[derive(Accounts)]
pub struct RemoveWhitelist<'info> {
    #[account(address = interest_depositor_manager.authority)]
    pub authority: Signer<'info>,

    #[account(address = interest_depositor_manager.mint)]
    pub mint: Box<Account<'info, Mint>>,
    
    #[account(
        mut,
        seeds = [
            SEED_INTEREST_DEPOSITOR_MANAGER,
            interest_depositor_manager.mint.key().as_ref(),
        ],
        bump = interest_depositor_manager.bump,
    )]
    pub interest_depositor_manager: Box<Account<'info, InterestDepositorManager>>,
}

pub fn remove_whitelist(
    ctx: Context<RemoveWhitelist>,
    whitelist_entry: Pubkey,
) -> Result<()> {
    let interest_depositor_manager = &mut ctx.accounts.interest_depositor_manager;
    let mint = &ctx.accounts.mint;

    if !interest_depositor_manager.whitelist.contains(&whitelist_entry) {
        return Err(SavingsVaultError::WhitelistEntryNotFound.into());
    }
    interest_depositor_manager.whitelist.retain(|e| e != &whitelist_entry); 
    
    emit!(RemoveWhitelistEvent {
        mint: mint.key(),
        whitelist_entry, 
    });

    Ok(())
}
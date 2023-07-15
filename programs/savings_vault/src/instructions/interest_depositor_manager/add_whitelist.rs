use {
    anchor_lang::prelude::*,
    anchor_spl::token::Mint,
    crate::{
        error::SavingsVaultError, 
        state::{
            InterestDepositorManager, 
            AddWhitelistEvent,
            SEED_INTEREST_DEPOSITOR_MANAGER, 
            WHITELIST_SIZE,
        }, 
    },
};


#[derive(Accounts)]
pub struct AddWhitelist<'info> {
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

pub fn add_whitelist(
    ctx: Context<AddWhitelist>,
    whitelist_entry: Pubkey,
) -> Result<()> {
    let interest_depositor_manager = &mut ctx.accounts.interest_depositor_manager;
    let mint = &ctx.accounts.mint;

    if interest_depositor_manager.whitelist.len() == WHITELIST_SIZE {
        return Err(SavingsVaultError::WhitelistFull.into());
    }

    interest_depositor_manager.whitelist.push(whitelist_entry);

    emit!(AddWhitelistEvent {
        mint: mint.key(),
        whitelist_entry, 
    });    

    Ok(())
}   
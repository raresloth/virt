use anchor_lang::prelude::*;
use anchor_spl::{
    token::{Mint},
};
use crate::{state::{Listing}, error::Error, MarketplaceConfig, CollectionConfig};

#[derive(Accounts)]
#[instruction(id: Pubkey)]
pub struct ListVirtual<'info> {
    /// Marketplace authority wallet.
    #[account(mut)]
    pub marketplace_authority: Signer<'info>,

    /// The currency to use or native mint if using SOL
    pub currency_mint: Box<Account<'info, Mint>>,

    #[account(
        init,
        seeds = [
            Listing::PREFIX.as_bytes(),
            id.as_ref()
        ],
        bump,
        space = Listing::SPACE,
        payer = marketplace_authority,
    )]
    pub listing: Box<Account<'info, Listing>>,

    #[account(
        seeds = [
            CollectionConfig::PREFIX.as_bytes(),
            collection_config.collection_mint.key().as_ref()
        ],
        bump = collection_config.bump[0],
        has_one = marketplace_authority,
    )]
    pub collection_config: Box<Account<'info, CollectionConfig>>,

    #[account(
        seeds = [
            MarketplaceConfig::PREFIX.as_bytes(),
            marketplace_config.marketplace_authority.key().as_ref()
        ],
        bump = marketplace_config.bump[0],
        has_one = marketplace_authority
    )]
    pub marketplace_config: Box<Account<'info, MarketplaceConfig>>,

    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn list_virtual_handler<'info>(
    ctx: Context<ListVirtual>,
    id: Pubkey,
    price: u64,
    expiry: i64,
) -> Result<()> {
    let listing = &mut ctx.accounts.listing;
    listing.init(
        [*ctx.bumps.get(Listing::PREFIX).ok_or(Error::BumpSeedNotInHashMap)?],
        ctx.accounts.marketplace_authority.key(),
        id,
        true,
        ctx.accounts.currency_mint.key(),
        ctx.accounts.collection_config.key(),
        ctx.accounts.marketplace_config.fee_config.clone(),
        price,
        expiry,
    )?;

    Ok(())
}
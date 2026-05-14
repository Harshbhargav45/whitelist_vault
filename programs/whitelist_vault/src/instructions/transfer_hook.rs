use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, TokenAccount};

use crate::{constants::*, WhitelistEntry};

#[derive(Accounts)]
pub struct TransferHook<'info> {
    #[account(token::mint = mint, token::authority = owner)]
    pub source_token: InterfaceAccount<'info, TokenAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(token::mint = mint)]
    pub destination_token: InterfaceAccount<'info, TokenAccount>,
    /// CHECK: source token account owner
    pub owner: UncheckedAccount<'info>,
    /// CHECK: ExtraAccountMetaList
    #[account(seeds = [b"extra-account-metas", mint.key().as_ref()], bump)]
    pub extra_account_meta_list: UncheckedAccount<'info>,
    #[account(
        seeds = [WHITELIST_SEED, owner.key().as_ref()],
        bump = whitelist_entry.bump,
    )]
    pub whitelist_entry: Account<'info, WhitelistEntry>,
}

pub fn transfer_hook_handler(_ctx: Context<TransferHook>, _amount: u64) -> Result<()> {
    Ok(())
}

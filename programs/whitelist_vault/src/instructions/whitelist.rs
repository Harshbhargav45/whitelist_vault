use anchor_lang::prelude::*;

use crate::{constants::*, error::VaultError, state::VaultConfig, WhitelistEntry};

#[derive(Accounts)]
#[instruction(user: Pubkey)]
pub struct AddToWhitelist<'info> {
    #[account(
        mut,
        constraint = admin.key() == vault_config.admin @ VaultError::UnauthorizedAdmin
    )]
    pub admin: Signer<'info>,

    #[account(seeds = [CONFIG_SEED], bump = vault_config.bump)]
    pub vault_config: Account<'info, VaultConfig>,

    #[account(
        init,
        payer = admin,
        space = 8 + WhitelistEntry::INIT_SPACE,
        seeds = [WHITELIST_SEED, user.as_ref()],
        bump,
    )]
    pub whitelist_entry: Account<'info, WhitelistEntry>,

    pub system_program: Program<'info, System>,
}

pub fn whitelist_handler(ctx: Context<AddToWhitelist>, user: Pubkey) -> Result<()> {
    ctx.accounts.whitelist_entry.user = user;
    ctx.accounts.whitelist_entry.deposited_amount = 0;
    ctx.accounts.whitelist_entry.bump = ctx.bumps.whitelist_entry;
    Ok(())
}

#[derive(Accounts)]
#[instruction(user: Pubkey)]
pub struct RemoveFromWhitelist<'info> {
    #[account(
        mut,
        constraint = admin.key() == vault_config.admin @ VaultError::UnauthorizedAdmin
    )]
    pub admin: Signer<'info>,

    #[account(seeds = [CONFIG_SEED], bump = vault_config.bump)]
    pub vault_config: Account<'info, VaultConfig>,

    #[account(
        mut,
        close = admin,
        seeds = [WHITELIST_SEED, user.as_ref()],
        bump = whitelist_entry.bump,
        constraint = whitelist_entry.deposited_amount == 0 @ VaultError::InsufficientBalance,
    )]
    pub whitelist_entry: Account<'info, WhitelistEntry>,
}

pub fn remove_whitelist_handler(_ctx: Context<RemoveFromWhitelist>, _user: Pubkey) -> Result<()> {
    Ok(())
}

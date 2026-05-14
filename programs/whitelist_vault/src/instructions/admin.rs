use anchor_lang::prelude::*;

use crate::{constants::*, error::VaultError, state::VaultConfig};

#[derive(Accounts)]
pub struct TogglePause<'info> {
    #[account(constraint = admin.key() == vault_config.admin @ VaultError::UnauthorizedAdmin)]
    pub admin: Signer<'info>,

    #[account(mut, seeds = [CONFIG_SEED], bump = vault_config.bump)]
    pub vault_config: Account<'info, VaultConfig>,
}

pub fn toggle_pause_handler(ctx: Context<TogglePause>) -> Result<()> {
    ctx.accounts.vault_config.paused = !ctx.accounts.vault_config.paused;
    Ok(())
}

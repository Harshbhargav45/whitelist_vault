use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface},
};
use spl_tlv_account_resolution::{
    account::ExtraAccountMeta, seeds::Seed, state::ExtraAccountMetaList,
};
use spl_transfer_hook_interface::instruction::ExecuteInstruction;

use crate::{constants::*, state::VaultConfig, WhitelistEntry};

#[derive(Accounts)]
#[instruction(deposit_cap: u64)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub admin: Signer<'info>,

    #[account(
        init,
        payer = admin,
        space = 8 + VaultConfig::INIT_SPACE,
        seeds = [CONFIG_SEED],
        bump,
    )]
    pub vault_config: Account<'info, VaultConfig>,

    #[account(
        init,
        payer = admin,
        space = 8 + WhitelistEntry::INIT_SPACE,
        seeds = [WHITELIST_SEED, VAULT_SEED],
        bump,
    )]
    pub vault_whitelist: Account<'info, WhitelistEntry>,

    /// CHECK: ExtraAccountMetaList
    #[account(
        mut,
        seeds = [b"extra-account-metas", mint.key().as_ref()],
        bump,
    )]
    pub extra_account_meta_list: UncheckedAccount<'info>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        init,
        payer = admin,
        token::mint = mint,
        token::authority = vault,
        seeds = [VAULT_SEED],
        bump,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn initialize_handler(ctx: Context<Initialize>, deposit_cap: u64) -> Result<()> {
    let config = &mut ctx.accounts.vault_config;
    config.admin = ctx.accounts.admin.key();
    config.mint = ctx.accounts.mint.key();
    config.total_deposits = 0;
    config.deposit_cap = deposit_cap;
    config.paused = false;
    config.bump = ctx.bumps.vault_config;

    ctx.accounts.vault_whitelist.user = ctx.accounts.vault.key();
    ctx.accounts.vault_whitelist.deposited_amount = 0;
    ctx.accounts.vault_whitelist.bump = ctx.bumps.vault_whitelist;

    let account_metas = vec![
        ExtraAccountMeta::new_with_seeds(
            &[
                Seed::Literal { bytes: WHITELIST_SEED.to_vec() },
                Seed::AccountKey { index: 3 },
            ],
            false,
            true,
        ).map_err(|_| ProgramError::InvalidAccountData)?,
    ];

    let account_size = ExtraAccountMetaList::size_of(account_metas.len())
        .map_err(|_| ProgramError::InvalidAccountData)?;
    let lamports = Rent::get()?.minimum_balance(account_size);

    let mint = ctx.accounts.mint.key();
    let signer_seeds: &[&[&[u8]]] = &[&[
        b"extra-account-metas",
        mint.as_ref(),
        &[ctx.bumps.extra_account_meta_list],
    ]];

    anchor_lang::system_program::create_account(
        CpiContext::new_with_signer(
            ctx.accounts.system_program.key(),
            anchor_lang::system_program::CreateAccount {
                from: ctx.accounts.admin.to_account_info(),
                to: ctx.accounts.extra_account_meta_list.to_account_info(),
            },
            signer_seeds,
        ),
        lamports,
        account_size as u64,
        &crate::ID,
    )?;

    ExtraAccountMetaList::init::<ExecuteInstruction>(
        &mut ctx.accounts.extra_account_meta_list.try_borrow_mut_data()?,
        &account_metas,
    ).map_err(|_| ProgramError::InvalidAccountData)?;

    Ok(())
}

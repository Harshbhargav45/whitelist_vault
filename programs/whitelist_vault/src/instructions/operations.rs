use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    Mint, MintTo, TokenAccount, TokenInterface, TransferChecked,
    mint_to, transfer_checked,
};

use crate::{
    constants::*,
    error::VaultError,
    state::{DepositEvent, VaultConfig, WithdrawEvent},
    WhitelistEntry,
};

#[derive(Accounts)]
pub struct MintToken<'info> {
    #[account(
        mut,
        constraint = admin.key() == vault_config.admin @ VaultError::UnauthorizedAdmin
    )]
    pub admin: Signer<'info>,

    #[account(seeds = [CONFIG_SEED], bump = vault_config.bump)]
    pub vault_config: Account<'info, VaultConfig>,

    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,

    /// CHECK: mint authority PDA
    #[account(seeds = [MINT_AUTH_SEED], bump)]
    pub mint_authority: UncheckedAccount<'info>,

    #[account(mut)]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut, seeds = [VAULT_SEED], bump)]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [WHITELIST_SEED, user.key().as_ref()],
        bump = user_whitelist.bump,
    )]
    pub user_whitelist: Account<'info, WhitelistEntry>,

    #[account(
        mut,
        seeds = [CONFIG_SEED],
        bump = vault_config.bump,
        constraint = !vault_config.paused @ VaultError::VaultPaused,
    )]
    pub vault_config: Account<'info, VaultConfig>,

    pub token_program: Interface<'info, TokenInterface>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = user,
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(mut, seeds = [VAULT_SEED], bump)]
    pub vault: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [WHITELIST_SEED, user.key().as_ref()],
        bump = user_whitelist.bump,
    )]
    pub user_whitelist: Account<'info, WhitelistEntry>,

    #[account(
        mut,
        seeds = [CONFIG_SEED],
        bump = vault_config.bump,
        constraint = !vault_config.paused @ VaultError::VaultPaused,
    )]
    pub vault_config: Account<'info, VaultConfig>,

    pub token_program: Interface<'info, TokenInterface>,
}

pub fn mint_token_handler(ctx: Context<MintToken>, amount: u64) -> Result<()> {
    require!(amount > 0, VaultError::ZeroAmount);

    let seeds: &[&[&[u8]]] = &[&[MINT_AUTH_SEED, &[ctx.bumps.mint_authority]]];

    mint_to(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.key(),
            MintTo {
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
                authority: ctx.accounts.mint_authority.to_account_info(),
            },
            seeds,
        ),
        amount,
    )?;

    Ok(())
}

pub fn deposit_handler(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    require!(amount > 0, VaultError::ZeroAmount);

    let config = &mut ctx.accounts.vault_config;
    let new_total = config.total_deposits
        .checked_add(amount)
        .ok_or(VaultError::Overflow)?;
    require!(new_total <= config.deposit_cap, VaultError::DepositCapExceeded);

    transfer_checked(
        CpiContext::new(
            ctx.accounts.token_program.key(),
            TransferChecked {
                from: ctx.accounts.user_token_account.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        amount,
        ctx.accounts.mint.decimals,
    )?;

    config.total_deposits = new_total;
    let whitelist = &mut ctx.accounts.user_whitelist;
    whitelist.deposited_amount = whitelist.deposited_amount
        .checked_add(amount)
        .ok_or(VaultError::Overflow)?;

    emit!(DepositEvent {
        user: ctx.accounts.user.key(),
        amount,
        total_user_deposits: whitelist.deposited_amount,
        total_vault_deposits: config.total_deposits,
    });

    Ok(())
}

pub fn withdraw_handler(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    require!(amount > 0, VaultError::ZeroAmount);

    let whitelist = &mut ctx.accounts.user_whitelist;
    require!(
        amount <= whitelist.deposited_amount,
        VaultError::InsufficientBalance
    );

    let seeds: &[&[&[u8]]] = &[&[VAULT_SEED, &[ctx.bumps.vault]]];

    transfer_checked(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.key(),
            TransferChecked {
                from: ctx.accounts.vault.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.user_token_account.to_account_info(),
                authority: ctx.accounts.vault.to_account_info(),
            },
            seeds,
        ),
        amount,
        ctx.accounts.mint.decimals,
    )?;

    whitelist.deposited_amount = whitelist.deposited_amount
        .checked_sub(amount)
        .ok_or(VaultError::Overflow)?;
    let config = &mut ctx.accounts.vault_config;
    config.total_deposits = config.total_deposits
        .checked_sub(amount)
        .ok_or(VaultError::Overflow)?;

    emit!(WithdrawEvent {
        user: ctx.accounts.user.key(),
        amount,
        remaining_user_deposits: whitelist.deposited_amount,
        total_vault_deposits: config.total_deposits,
    });

    Ok(())
}

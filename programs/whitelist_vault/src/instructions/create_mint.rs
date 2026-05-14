use anchor_lang::prelude::*;
use anchor_spl::token_interface::spl_token_2022;
use anchor_lang::solana_program::{program::invoke, program_pack::Pack};

use crate::constants::*;

#[derive(Accounts)]
pub struct CreateMint<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,

    /// CHECK: initialized as mint via CPI
    #[account(
        init,
        payer = payer,
        space = spl_token_2022::state::Mint::LEN + 200,
        owner = token_program.key(),
        seeds = [MINT_SEED],
        bump,
    )]
    pub mint: UncheckedAccount<'info>,

    /// CHECK: mint authority PDA
    #[account(seeds = [MINT_AUTH_SEED], bump)]
    pub mint_authority: UncheckedAccount<'info>,

    pub token_program: Interface<'info, anchor_spl::token_interface::TokenInterface>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn create_mint(ctx: Context<CreateMint>) -> Result<()> {
    let mint = ctx.accounts.mint.to_account_info();
    let token_program = ctx.accounts.token_program.to_account_info();
    let mint_authority = ctx.accounts.mint_authority.key();

    invoke(
        &spl_token_2022::extension::transfer_hook::instruction::initialize(
            &token_program.key(),
            &mint.key(),
            Some(crate::ID),
            Some(crate::ID),
        )?,
        &[mint.clone()],
    )?;

    invoke(
        &spl_token_2022::instruction::initialize_permanent_delegate(
            &token_program.key(),
            &mint.key(),
            &crate::ID,
        )?,
        &[mint.clone()],
    )?;

    invoke(
        &spl_token_2022::instruction::initialize_mint(
            &token_program.key(),
            &mint.key(),
            &mint_authority,
            None,
            9,
        )?,
        &[mint.clone(), ctx.accounts.rent.to_account_info()],
    )?;

    Ok(())
}

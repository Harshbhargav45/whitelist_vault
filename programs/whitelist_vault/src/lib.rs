use anchor_lang::prelude::*;

pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

pub use constants::*;
pub use state::*;
pub use instructions::*;

declare_id!("5tkv4vBNjVaceh7MTUdNaGCstuthRtWw8HL99r2LezaG");

#[program]
pub mod whitelist_vault {
    use super::*;

    pub fn init_mint(ctx: Context<CreateMint>) -> Result<()> {
        instructions::create_mint::create_mint(ctx)
    }

    pub fn init_vault(ctx: Context<Initialize>, deposit_cap: u64) -> Result<()> {
        instructions::initialize::initialize_handler(ctx, deposit_cap)
    }

    pub fn add_to_whitelist(ctx: Context<AddToWhitelist>, user: Pubkey) -> Result<()> {
        instructions::whitelist::whitelist_handler(ctx, user)
    }

    pub fn remove_from_whitelist(ctx: Context<RemoveFromWhitelist>, user: Pubkey) -> Result<()> {
        instructions::whitelist::remove_whitelist_handler(ctx, user)
    }

    pub fn mint_token(ctx: Context<MintToken>, amount: u64) -> Result<()> {
        instructions::operations::mint_token_handler(ctx, amount)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        instructions::operations::deposit_handler(ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        instructions::operations::withdraw_handler(ctx, amount)
    }

    pub fn toggle_pause(ctx: Context<TogglePause>) -> Result<()> {
        instructions::admin::toggle_pause_handler(ctx)
    }

    #[instruction(discriminator = [105, 37, 101, 197, 75, 251, 102, 26])]
    pub fn transfer_hook(ctx: Context<TransferHook>, amount: u64) -> Result<()> {
        instructions::transfer_hook::transfer_hook_handler(ctx, amount)
    }
}

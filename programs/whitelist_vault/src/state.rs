use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct VaultConfig {
    pub admin: Pubkey,
    pub mint: Pubkey,
    pub total_deposits: u64,
    pub deposit_cap: u64,
    pub paused: bool,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct WhitelistEntry {
    pub user: Pubkey,
    pub deposited_amount: u64,
    pub bump: u8,
}

#[event]
pub struct DepositEvent {
    pub user: Pubkey,
    pub amount: u64,
    pub total_user_deposits: u64,
    pub total_vault_deposits: u64,
}

#[event]
pub struct WithdrawEvent {
    pub user: Pubkey,
    pub amount: u64,
    pub remaining_user_deposits: u64,
    pub total_vault_deposits: u64,
}

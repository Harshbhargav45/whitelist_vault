use anchor_lang::prelude::*;

#[error_code]
pub enum VaultError {
    #[msg("User is not whitelisted")]
    NotWhitelisted,
    #[msg("Vault is currently paused")]
    VaultPaused,
    #[msg("Insufficient deposited balance for withdrawal")]
    InsufficientBalance,
    #[msg("Deposit would exceed the vault cap")]
    DepositCapExceeded,
    #[msg("Unauthorized: signer is not the admin")]
    UnauthorizedAdmin,
    #[msg("Amount must be greater than zero")]
    ZeroAmount,
    #[msg("Arithmetic overflow")]
    Overflow,
}

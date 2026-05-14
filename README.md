# Whitelist Vault with Transfer Hook

A Solana smart contract that implements a token vault with whitelisting functionality and token-2022 transfer hooks.

## Features

- **Mint Initialization**: Create a token-2022 mint with the transfer hook extension natively enabled.
- **Vault Configuration**: Initialize a secure vault with an admin-defined total deposit cap.
- **PDA-Based Whitelisting**: Store whitelisted user status in individual PDA accounts for efficient compute usage and targeted verification.
- **Deposit & Withdrawal Operations**: Securely deposit tokens to the vault and withdraw them while checking global and individual constraints.
- **Transfer Hooks**: Intercept token transfers using Token Extensions to dynamically resolve the whitelisted PDA status of involved parties.
- **Admin Controls**: Provides the ability for the admin to pause operations globally.

## Project Structure

- `programs/whitelist_vault/src/lib.rs`: The main entrypoint for the Anchor program.
- `programs/whitelist_vault/src/state.rs`: Defines the state accounts (VaultConfig, WhitelistEntry) and events.
- `programs/whitelist_vault/src/instructions/`: Contains the handler logic for all the operations.

## Testing

The project is structured with an integration test suite using LiteSVM to simulate the Solana runtime and verify execution.

To run tests locally:
```sh
cargo test
```

## Build

To build the program and generate the IDL:
```sh
anchor build
```

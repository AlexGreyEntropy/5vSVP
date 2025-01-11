// Percentage of royalties that go into the vault (20%)
pub const VAULT_FEE_PERCENTAGE: u64 = 20;

// Percentage of royalties that go to the creator (80%)
pub const CREATOR_FEE_PERCENTAGE: u64 = 80;

// The space required for the Vault account in bytes
pub const VAULT_ACCOUNT_SIZE: usize = 8 + // discriminator
                                      32 + // collection Pubkey
                                      32 + // mint Pubkey
                                      32 + // token_account Pubkey
                                      32 + // owner Pubkey
                                      8;   // escrow_balance

pub const VAULT_SEED: &[u8] = b"vault";

// Bump seed length for PDAs
pub const BUMP_SEED_LENGTH: usize = 1;

// Maximum allowed amount for transactions (in lamports, can be adjusted based on your use case)
pub const MAX_TRANSACTION_AMOUNT: u64 = 1_000_000_000_000; // 1,000 SOL in lamports

// Timeout for operations that might require waiting
pub const OPERATION_TIMEOUT: u64 = 300; // seconds 

/// Minimum amount of SOL (in lamports) required to create a vault
pub const MIN_VAULT_CREATION_AMOUNT: u64 = 1_000_000; // 0.001 SOL in lamports
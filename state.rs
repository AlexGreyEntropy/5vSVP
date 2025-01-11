use anchor_lang::prelude::*;

// Represents the Vault structure used for storing and managing SOL related to an NFT.
#[account]
pub struct Vault {
    pub collection: Pubkey,
    pub mint: Pubkey,
    pub token_account: Pubkey,
    pub owner: Pubkey,
    pub escrow_balance: u64,
}

impl Vault {
    pub fn new(collection: Pubkey, mint: Pubkey, token_account: Pubkey, owner: Pubkey) -> Self {
        Vault {
            collection,
            mint,
            token_account,
            owner,
            escrow_balance: 0,
        }
    }

    // Updates the owner of the vault. Should be called when the NFT is transferred/transacted
    pub fn update_owner(&mut self, new_owner: Pubkey) {
        self.owner = new_owner;
    }

    // Adds SOL to the vault's escrow balance, and called when a royalty fee is processed.
    pub fn add_to_escrow(&mut self, amount: u64) {
        self.escrow_balance = self.escrow_balance.checked_add(amount).unwrap();
    }

    // Releases all funds held in escrow back to the current owner, and called when the NFT is returned to the collection address.
    pub fn release_escrow(&mut self) -> u64 {
        let amount = self.escrow_balance;
        self.escrow_balance = 0;
        amount
    }
}
use anchor_lang::prelude::*;
use super::state::Vault;
use super::errors::SuperVaultError;

pub fn initialize_vault(
    ctx: Context<InitializeVault>,
    amount: u64,
) -> Result<()> {
    // new vault account with space for the Vault struct
    let vault = &mut ctx.accounts.vault;
    *vault = Vault::new(
        ctx.accounts.collection.key(),
        ctx.accounts.asset.mint,
        ctx.accounts.asset.token_account,
        ctx.accounts.payer.key(),
    );

    // Initial transfer of SOL amount to the vault from minting price.
    anchor_lang::system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.payer.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
            },
        ),
        amount,
    )?;

    // Log the initialization for debugging
    msg!("Vault initialized with {} SOL", amount);
    Ok(())
}

pub fn update_vault_owner(ctx: Context<UpdateVaultOwner>) -> Result<()> {
    // update the vault's owner when the NFT changes hands
    let vault = &mut ctx.accounts.vault;
    vault.owner = ctx.accounts.new_owner.key();
    
    // log owner update
    msg!("Vault owner updated to: {}", vault.owner);
    Ok(())
}

pub fn process_royalty(ctx: Context<ProcessRoyalty>, amount: u64) -> Result<()> {
    // calculate royalty distribution !!!
    let to_vault = amount ;
    let to_creator = amount - to_vault;

    //update the vault's escrow balance with 100 seller basis fee points
    let vault = &mut ctx.accounts.vault;
    vault.escrow_balance += to_vault;

    // transfer remaining to the creator
    anchor_lang::system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.payer.to_account_info(),
                to: ctx.accounts.creator.to_account_info(),
            },
        ),
        to_creator,
    )?;

    // Log the royalty distribution
    msg!("Royalty processed: {} to vault, {} to creator", to_vault, to_creator);
    Ok(())
}

pub fn release_funds(ctx: Context<ReleaseFunds>) -> Result<()> {
    // Check if the NFT is back in the collection before releasing funds !!s
    if ctx.accounts.asset.owner != ctx.accounts.collection.key() {
        return Err(error!(SuperVaultError::NotInCollection));
    }

    // Release the funds in the vault back to the last known owner
    let vault = &mut ctx.accounts.vault;
    let amount = vault.escrow_balance;
    vault.escrow_balance = 0;  // Reset the balance after transfer

    anchor_lang::system_program::transfer(
        CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.vault.to_account_info(),
                to: ctx.accounts.last_owner.to_account_info(),
            },
        ),
        amount,
    )?;

    // Log the fund release
    msg!("Funds released: {} SOL to {}", amount, ctx.accounts.last_owner.key());
    Ok(())
}
use crate::event;
use anchor_lang::prelude::*;

/// Accounts for [amm::move_locked_lp].
#[derive(Accounts)]
pub struct MoveLockedLp<'info> {
    /// Pool account
    /// CHECK: This account is validated in the handler
    #[account(mut)]
    pub pool: UncheckedAccount<'info>,

    /// LP token mint of the pool
    /// CHECK: This account is validated in the handler
    pub lp_mint: UncheckedAccount<'info>,

    /// From lock account
    /// CHECK: This account is validated in the handler
    #[account(mut)]
    pub from_lock_escrow: UncheckedAccount<'info>,

    /// To lock account
    /// CHECK: This account is validated in the handler
    #[account(mut)]
    pub to_lock_escrow: UncheckedAccount<'info>,

    /// Owner of lock account
    pub owner: Signer<'info>,

    /// From escrow vault
    /// CHECK: This account is validated in the handler
    #[account(mut)]
    pub from_escrow_vault: UncheckedAccount<'info>,

    /// To escrow vault
    /// CHECK: This account is validated in the handler
    #[account(mut)]
    pub to_escrow_vault: UncheckedAccount<'info>,

    /// Token program.
    /// CHECK: This account is the SPL token program
    pub token_program: UncheckedAccount<'info>,

    // /// Vault account for token a. token a of the pool will be deposit / withdraw from this vault account.
    /// CHECK: This account is validated in the handler
    pub a_vault: UncheckedAccount<'info>,
    // /// Vault account for token b. token b of the pool will be deposit / withdraw from this vault account.
    /// CHECK: This account is validated in the handler
    pub b_vault: UncheckedAccount<'info>,
    // /// LP token account of vault A. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    /// CHECK: This account is validated in the handler
    pub a_vault_lp: UncheckedAccount<'info>,
    // /// LP token account of vault B. Used to receive/burn the vault LP upon deposit/withdraw from the vault.
    /// CHECK: This account is validated in the handler
    pub b_vault_lp: UncheckedAccount<'info>,
    // /// LP token mint of vault a
    /// CHECK: This account is validated in the handler
    pub a_vault_lp_mint: UncheckedAccount<'info>,
    // /// LP token mint of vault b
    /// CHECK: This account is validated in the handler
    pub b_vault_lp_mint: UncheckedAccount<'info>,
}

/// move locked lp
pub fn move_locked_lp(ctx: Context<MoveLockedLp>, max_amount: u64) -> Result<()> {
    Ok(())
}

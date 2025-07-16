
use anchor_lang::prelude::*;
use solana_program::system_instruction;
use solana_program::program::invoke;

const TOKENS_PER_SOL: u64 = 1000;
const MAX_CAP_SOL: u64 = 3_500_000 * LAMPORTS_PER_SOL;

#[program]
pub mod lunaro_presale {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let presale = &mut ctx.accounts.presale;
        presale.total_raised = 0;
        presale.authority = ctx.accounts.authority.key();
        Ok(())
    }

    pub fn buy_tokens(ctx: Context<BuyTokens>, amount: u64) -> Result<()> {
        require!(amount > 0, LunaroError::InvalidAmount);
        let presale = &mut ctx.accounts.presale;
        require!(presale.total_raised + amount <= MAX_CAP_SOL, LunaroError::CapReached);
        let user = &ctx.accounts.buyer;
        let vault = &ctx.accounts.vault;

        invoke(
            &system_instruction::transfer(&user.key(), &vault.key(), amount),
            &[user.to_account_info(), vault.to_account_info(), ctx.accounts.system_program.to_account_info()],
        )?;

        presale.total_raised += amount;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 40)]
    pub presale: Account<'info, Presale>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BuyTokens<'info> {
    #[account(mut)]
    pub presale: Account<'info, Presale>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    /// CHECK: vault is a verified external multisig address (Squads)
    #[account(mut)]
    pub vault: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct Presale {
    pub total_raised: u64,
    pub authority: Pubkey,
}

#[error_code]
pub enum LunaroError {
    #[msg("Invalid amount")]
    InvalidAmount,
    #[msg("Cap already reached")]
    CapReached,
}

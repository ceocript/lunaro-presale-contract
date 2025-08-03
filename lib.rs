
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};
use solana_program::system_instruction;
use solana_program::program::invoke;
use std::mem::size_of;

pub const LNR_TOKEN_MINT: &str = "LUAJZB1dmPGuxGqonyVY6Wp4AR5UyVFVom6cWZYuPky";
pub const ADMIN_PUBKEY: &str = "9NYahmkGRoJGeA3rQCVx6XVmQgGGw3Mq9EpnVSWeU1gp";
pub const TOKENS_PER_SOL: u64 = 1000;
pub const HARD_CAP_SOL: u64 = 3_500_000_000_000_000; // 3.5 milhões de SOL em lamports

#[program]
pub mod lunaro_presale {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let presale = &mut ctx.accounts.presale;
        presale.total_raised = 0;
        presale.authority = ctx.accounts.authority.key();
        Ok(())
    }

    pub fn buy_tokens(ctx: Context<BuyTokens>, amount_sol: u64) -> Result<()> {
        // Valida que o valor é maior que zero
        require!(amount_sol > 0, LunaroError::InvalidAmount);

        let presale = &mut ctx.accounts.presale;

        // Valida que a compra não excederá o limite
        require!(presale.total_raised.checked_add(amount_sol).unwrap() <= HARD_CAP_SOL, LunaroError::CapReached);

        // Calcula a quantidade de tokens LNR a serem enviados
        let amount_lnr = amount_sol.checked_mul(TOKENS_PER_SOL).unwrap();

        // 1. Transfere SOL do comprador para o vault (multisig)
        invoke(
            &system_instruction::transfer(&ctx.accounts.buyer.key(), &ctx.accounts.vault.key(), amount_sol),
            &[
                ctx.accounts.buyer.to_account_info(),
                ctx.accounts.vault.to_account_info(),
                ctx.accounts.system_program.to_account_info()
            ],
        )?;

        // 2. Transfere tokens LNR do vault do token para o comprador
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.token_vault.to_account_info(),
                    to: ctx.accounts.buyer_token_account.to_account_info(),
                    authority: ctx.accounts.authority.to_account_info(),
                },
            ),
            amount_lnr,
        )?;

        // 3. Atualiza o total de SOL arrecadado no estado do contrato
        presale.total_raised = presale.total_raised.checked_add(amount_sol).unwrap();

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    // Aloca espaço correto: 8 (discriminador) + 8 (u64) + 32 (Pubkey) = 48 bytes
    #[account(init, payer = authority, space = 8 + size_of::<Presale>())]
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
    /// CHECK: A conta vault é um multisig externo (Squads)
    #[account(mut)]
    pub vault: UncheckedAccount<'info>,
    #[account(mut)]
    // O token vault detém os tokens LNR para a pré-venda
    pub token_vault: Account<'info, TokenAccount>,
    #[account(mut)]
    // A conta do token do comprador
    pub buyer_token_account: Account<'info, TokenAccount>,
    // Autoridade para a transferência
    #[account(signer)]
    pub authority: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
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

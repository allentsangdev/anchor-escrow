//! Sol Escrow
//! Initializer can create a SOL Escrow.
//! Initializer will deposit SOL to the Escrow while creating it.
//! Initializer will specify who can redeem the Escrow.
//! The Escrow Account will take ownership of the SOL.
//! Taker can call the redeem function to redeem SOL

use anchor_lang::prelude::*;
use anchor_lang::solana_program::program::invoke;
use anchor_lang::solana_program::system_instruction::transfer;
use solana_program::entrypoint::ProgramResult;

declare_id!("4n2mE5KrT4nBHeM1zeVV1NNLNh6if46T6S69jqGFF9P4");

#[program]
pub mod project {
    use super::*;

    pub fn init_sol_escrow(ctx: Context<InitSolEscrow>, amount: u64, to: Pubkey) -> ProgramResult {
        // Deposit SOL to the escrow
        let ix = transfer(&ctx.accounts.user.key(), &ctx.accounts.escrow.key(), amount);

        // Invoke the instruction
        invoke(
            &ix,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.escrow.to_account_info(),
            ],
        )?;

        // Update the escrow account
        let escrow = &mut ctx.accounts.escrow;
        escrow.balance = amount;
        escrow.from = ctx.accounts.user.key();
        escrow.to = to;

        Ok(())
    }

    pub fn redeem(ctx: Context<Redeem>) -> ProgramResult {
        let escrow = &mut ctx.accounts.escrow;

        // Ensure the caller is the intended redeemer
        // require!(
        //     escrow.to == ctx.accounts.user.key(),
        //     EscrowError::InvalidRedeemer
        // );

        if escrow.to != ctx.accounts.user.key() {
            return Err(ProgramError::InvalidAccountData.into());
        }

        // Transfer SOL from the escrow to the user
        let ix = transfer(&escrow.key(), &ctx.accounts.user.key(), escrow.balance);
        invoke(
            &ix,
            &[
                escrow.to_account_info(),
                ctx.accounts.user.to_account_info(),
            ],
        )?;

        // Update the escrow account
        escrow.to = ctx.accounts.user.key();
        escrow.balance = 0;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitSolEscrow<'info> {
    #[account(init, payer = user, space = 8 + 8 + 32 + 32)]
    pub escrow: Account<'info, SolEscrow>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct SolEscrow {
    pub balance: u64,
    pub from: Pubkey,
    pub to: Pubkey,
}

#[derive(Accounts)]
pub struct Redeem<'info> {
    #[account(mut)]
    pub escrow: Account<'info, SolEscrow>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum EscrowError {
    #[msg("The caller is not authorized to redeem this escrow.")]
    InvalidRedeemer,
}

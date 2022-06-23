use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod anchor_escrow {
    use anchor_lang::solana_program::entrypoint::ProgramResult;

    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        _vault_account_bump: u8,
        initializer_amount: u64,
        taker_amount: u64
    ) -> ProgramResult {
        
        Ok(())
    }

    pub fn cancel(ctx: Context<Cancel>) -> ProgramResult {
        Ok(())
    }

    pub fn exchange(ctx: Context<Exchange>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {

}

#[derive(Accounts)]
pub struct Exchange<'info> {

}

#[derive(Accounts)]
pub struct Cancel<'info> {

}

#[account]
pub struct EscrowAccount {
    pub initializer_key: Pubkey,
    pub initializer_deposit_token_account: Pubkey,
    pub initializer_receive_token_account: Pubkey,
    pub initializer_amount: u64,
    pub taker_amount: u64,
}
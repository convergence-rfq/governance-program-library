use anchor_lang::prelude::*;

mod error;
// use error::*;

mod instructions;
use instructions::*;

mod governance;
mod state;

// use state::*;
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod nft_voter {

    use super::*;
    pub fn create_registrar(ctx: Context<CreateRegistrar>) -> Result<()> {
        instructions::create_registrar(ctx)
    }
    pub fn create_voter_weight_record(
        ctx: Context<CreateVoterWeightRecord>,
        governing_token_owner: Pubkey,
    ) -> Result<()> {
        instructions::create_voter_weight_record(ctx, governing_token_owner)
    }
    pub fn create_max_voter_weight_record(ctx: Context<CreateMaxVoterWeightRecord>) -> Result<()> {
        instructions::create_max_voter_weight_record(ctx)
    }
    pub fn update_voter_weight_record(
        ctx: Context<UpdateVoterWeightRecord>,
        realm: Pubkey,
        governing_token_mint: Pubkey,
        governing_token_owner: Pubkey,
    ) -> Result<()> {
        instructions::update_voter_weight_record(
            ctx,
            realm,
            governing_token_mint,
            governing_token_owner,
        )
    }
    pub fn relinquish_vote(
        ctx: Context<RelinquishVote>,
        realm: Pubkey,
        governing_token_mint: Pubkey,
        governing_token_owner: Pubkey,
    ) -> Result<()> {
        instructions::relinquish_vote(ctx, realm, governing_token_mint, governing_token_owner)
    }
}

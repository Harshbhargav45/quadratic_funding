use anchor_lang::prelude::*;
use anchor_lang::Discriminator;
use anchor_spl::token_interface::Mint;

use crate::Dao;

#[derive(Accounts)]
#[instruction(name: String)]
pub struct InitDao<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    #[account(
        init,
        payer = creator,
        space = Dao::DISCRIMINATOR.len() + Dao::INIT_SPACE,
        seeds = [b"dao", creator.key().as_ref(), name.as_bytes()],
        bump,
    )]
    pub dao_account: Account<'info, Dao>,

    pub mint: InterfaceAccount<'info, Mint>,

    pub system_program: Program<'info, System>,
}

impl<'info> InitDao<'info> {
    pub fn init_dao(&mut self, name: String, bumps: &InitDaoBumps) -> Result<()> {
        self.dao_account.set_inner(Dao {
            name,
            authority: self.creator.key(),
            mint: self.mint.key(),
            proposal_count: 0,
            bump: bumps.dao_account,
        });

        Ok(())
    }
}
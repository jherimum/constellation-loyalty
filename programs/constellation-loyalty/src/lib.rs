use anchor_lang::prelude::*;

declare_id!("45JJwBkjECvyEPH4Keai5623Sr2z5GdYDWhoXZYbhPfh");

#[program]
pub mod constellation_loyalty {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

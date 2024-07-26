use anchor_lang::prelude::*;

declare_id!("38uoZRhNUUrrbWpQutZ8fQhAQhZytFygVaqGrdjHDdUp");

#[program]
pub mod rate_limited_oracle {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

pub mod errors;
pub mod state;
use anchor_lang::prelude::*;

use crate::{state::*, errors::*};

declare_id!("38uoZRhNUUrrbWpQutZ8fQhAQhZytFygVaqGrdjHDdUp");

const MAX_CALLS: u8 = 3;

#[program]
pub mod rate_limited_oracle {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, new_period: i64, new_price: u64) -> Result<()> {
        let oracle = &mut ctx.accounts.oracle;
        let current_time = Clock::get()?.unix_timestamp;
        oracle.time = current_time;
        oracle.price = new_price;
        oracle.period = new_period;
        oracle.admin = ctx.accounts.admin.key();
        Ok(())
    }

    pub fn update_price(ctx: Context<UpdatePrice>, new_price: u64) -> Result<()> {
        let oracle = &mut ctx.accounts.oracle;
        let rate_limit = &mut ctx.accounts.rate_limit;
        let current_time = Clock::get()?.unix_timestamp;

        if current_time - rate_limit.last_called >= oracle.period {
            rate_limit.calls = 0;
            rate_limit.last_called = current_time;
        }

        if rate_limit.calls >= MAX_CALLS {
            return Err(OracleError::RateLimitExceeded.into());
        }

        oracle.price = new_price;
        oracle.time = current_time;

        rate_limit.user = ctx.accounts.user.key();
        rate_limit.last_called = current_time;
        rate_limit.calls += 1;

        Ok(())
    }

    pub fn update_period(ctx: Context<UpdatePeriod>, new_period: i64) -> Result<()> {
        let oracle: &mut Account<Oracle> = &mut ctx.accounts.oracle;
        oracle.period = new_period;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init, 
        payer = admin, 
        space = 8 + std::mem::size_of::<Oracle>(),
        seeds = [b"oracle".as_ref()],
        bump
    )]
    pub oracle: Account<'info, Oracle>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdatePrice<'info> {
    #[account(mut)]
    pub oracle: Account<'info, Oracle>,
    #[account(
        init_if_needed, 
        payer = user, 
        space = 8 + std::mem::size_of::<RateLimit>(),
        seeds = [b"rate_limit".as_ref(), user.key().as_ref()],
        bump,
    )]
    pub rate_limit: Account<'info, RateLimit>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdatePeriod<'info> {
    #[account(mut)]
    pub oracle: Account<'info, Oracle>,
    #[account(
        mut,
        constraint = user.key() == oracle.admin
    )]
    pub user: Signer<'info>,
}

use anchor_lang::prelude::*;

#[account]
pub struct Oracle {
    pub price: u64,
    pub time: i64,
}

#[account]
pub struct RateLimit {
    pub calls: u8,
    pub last_called: i64,
    pub user: Pubkey,
}

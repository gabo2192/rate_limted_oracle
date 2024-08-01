use anchor_lang::prelude::*;

#[error_code]
pub enum OracleError {
    #[msg("Rate limit exceeded")]
    RateLimitExceeded,
    #[msg("Unauthorized")]
    Unauthorized,
}

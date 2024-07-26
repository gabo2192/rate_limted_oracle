use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("Rate limit exceeded")]
    RateLimitExceeded,
    #[msg("Unauthorized")]
    Unauthorized,
}

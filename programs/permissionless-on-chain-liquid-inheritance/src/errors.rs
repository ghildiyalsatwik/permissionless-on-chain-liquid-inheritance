use anchor_lang::error_code;

#[error_code]
pub enum ProtocolError {
    #[msg("InvalidAdmin")]
    InvalidAdmin,
    #[msg("ProtocolUnlocked")]
    ProtocolUnlocked,
    #[msg("ProtocolLocked")]
    ProtocolLocked,
    #[msg("InvalidMintAccount")]
    InvalidMintAccount,
    #[msg("MathOverflow")]
    MathOverflow,
    #[msg("TimeElapsed")]
    TimeElapsed,
    #[msg("InvalidMaker")]
    InvalidMaker,
    #[msg("InvalidTimestamp")]
    InvalidTimestamp
}

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
    #[msg("TimeNotElapsed")]
    TimeNotElapsed,
    #[msg("InvalidMaker")]
    InvalidMaker,
    #[msg("InvalidTimestamp")]
    InvalidTimestamp,
    #[msg("InvalidTokenAmount")]
    InvalidTokenAmount,
    #[msg("InvalidInstruction")]
    InvalidInstruction,
    #[msg("NoSharesAvailable")]
    NoSharesAvailable,
    #[msg("InvalidState")]
    InvalidState,
    #[msg("InvalidInheritor")]
    InvalidInheritor,
    #[msg("InvalidProtocolProgram")]
    InvalidProgram,
    #[msg("InvalidTokenProgram")]
    InvalidTokenProgram
}

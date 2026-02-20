use anchor_lang::error_code;

#[error_code]
pub enum ProtocolError {
    #[msg("DefaultError")]
    DefaultError,
}

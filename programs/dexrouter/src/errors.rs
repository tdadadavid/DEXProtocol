use anchor_lang::error_code;

#[error_code]
pub enum ErrorCode {
    #[msg("overflow during math calculation")]
    Overflow,

    #[msg("Divide by zero error")]
    DivideByZero,

    #[msg("invalid input provided")]
    InvalidInput
}
use anchor_lang::prelude::*;

/// Simulate Constant Product AMM (k = x * y)
///
/// X -> token_a
/// y -> token_b
/// dx -> amount_in
///
pub fn get_output_amount(x: u64, y: u64, dx: u64) -> Result<u64> {
    let k = x.checked_mul(y).ok_or(ErrorCode::Overflow)?;
    let new_x = x.checked_add(dx).ok_or(ErrorCode::Overflow)?;
    let new_y = k.checked_div(new_x).ok_or(ErrorCode::DivideByZero)?;
    let dy  = y.checked_sub(new_y).ok_or(ErrorCode::Overflow)?;
    Ok(dy)
}

#[error_code]
pub enum ErrorCode {
    #[msg("overflow during math calculation")]
    Overflow,
    #[msg("Divide by zero error")]
    DivideByZero
}
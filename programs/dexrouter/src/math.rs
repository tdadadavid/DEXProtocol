use anchor_lang::prelude::*;

pub const SCALE: u128 = 1_000_000_000_000_000_000;

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


/// Formula
/// amountOut = y * (1-(x/x+amountIn)^(wx/wy))
/// Where
///   x = token_a or token_in reserve
///   y = token_b or token_out reserve
///   amountIn = The amount of token_a deposited to the pool in exchange for token_b
///   wx = weight of token_a
///   wy = weight of token_b
///
pub fn get_weighted_amount(
  token_in_balance: u128,
  token_out_balance: u128,
  weight_a: u128,
  weight_b: u128,
  amount_in: u128
) -> Result<u64> {
  // 1. Validations
  if amount_in == 0 || token_in_balance == 0 || token_out_balance == 0 {
    return Ok(0);
  }

  // Base (x/x + amountIn)
  let base = token_in_balance
    .checked_div(
      token_in_balance + amount_in
    )
    .ok_or(ErrorCode::DivideByZero)?;

  // Exponent (wx/wy)
  let exponent = weight_a
    .checked_div(weight_b)
    .ok_or(ErrorCode::DivideByZero)?;

  // base ^ exponent
  let power = pow_fixed(base, exponent)?;

  // 1 - (base ^ exponent)
  let multiplier = SCALE.checked_sub(power).ok_or(ErrorCode::Overflow)?;

  // amount_out = token_out_balance * multiplier
  let amount_out = (token_out_balance * multiplier)
    .checked_div(SCALE)
    .ok_or(ErrorCode::Overflow)?;

  Ok(amount_out as u64)
}


pub fn pow_fixed(base: u128, exp: u128) -> Result<u128> {
    if base == 0 {
        return Ok(0);
    }

    // Convert base to natural log using series approximation
    // ln(x) ≈ 2 * (x - 1) / (x + 1) for x close to 1
    // We assume base is in range [0.5, 1], else clamp or scale
    if base > SCALE {
        return Err(ErrorCode::InvalidInput.into());
    }

    // Simple ln(x) approximation: ln(x) ≈ (x - 1)
    // (in real usage you'd use a series like log2 or natural log approximations)
    let ln_base = base.checked_sub(SCALE).ok_or(ErrorCode::Overflow)?;

    // log(x^y) = y * log(x)
    let ln_result = ln_base
        .checked_mul(exp)
        .ok_or(ErrorCode::Overflow)?
        .checked_div(SCALE)
        .ok_or(ErrorCode::DivideByZero)?;

    // exp(x) ≈ 1 + x + x^2/2! + x^3/3! (for small x)
    let mut result = SCALE;
    let mut term = SCALE;
    let mut x = ln_result;
    let mut i = 1;

    // Limit to 10 iterations
    for _ in 0..10 {
        term = term
            .checked_mul(x)
            .ok_or(ErrorCode::Overflow)?
            .checked_div(SCALE * i as u128)
            .ok_or(ErrorCode::DivideByZero)?;
        result = result.checked_add(term).ok_or(ErrorCode::Overflow)?;
        i += 1;
    }

    Ok(result)
}

#[error_code]
pub enum ErrorCode {
    #[msg("overflow during math calculation")]
    Overflow,

    #[msg("Divide by zero error")]
    DivideByZero,

    #[msg("invalid input provided")]
    InvalidInput
}
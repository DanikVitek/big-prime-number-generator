use anyhow::anyhow;
use num_bigint::BigUint;
use num_traits::{One, Zero};

pub fn log2(num: &BigUint) -> anyhow::Result<u64> {
    let num = num.clone();
    if num.is_zero() {
        return Err(anyhow!("Number must be positive"));
    }
    if num.is_one() {
        return Ok(0);
    }
    Ok(num.to_str_radix(2).len() as u64 - 1)
}

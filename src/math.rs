use anyhow::anyhow;
use num_bigint::ToBigUint;
use num_traits::{One, Zero};

pub fn log2<N>(num: &N) -> anyhow::Result<u64>
where
    N: ToBigUint,
{
    let mut num = num
        .to_biguint()
        .ok_or(anyhow!("Number must be a BigUInt"))?;
    if num.is_zero() {
        return Err(anyhow!("Number must be positive"));
    }
    if num.is_one() {
        return Ok(0);
    }
    let mut log: u64 = 0;
    while !num.is_one() {
        num /= 2u8;
        log += 1;
    }
    Ok(log)
}

#[test]
fn _64log2eq6() {
    assert_eq!(6u64, log2(&64).unwrap())
}

#[test]
fn _100log2eq6() {
    assert_eq!(6u64, log2(&100).unwrap())
}

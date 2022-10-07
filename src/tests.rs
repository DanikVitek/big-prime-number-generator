use std::num::{NonZeroI64, NonZeroU64};

use anyhow::anyhow;
use num_bigint::{BigUint, RandomBits, ToBigUint};
use num_traits::Zero;
use rand::Rng;

use crate::math::log2;

const PRIME_NUMBERS: [u8; 8] = [2, 3, 5, 7, 11, 13, 17, 23];

pub enum TestResult {
    NotBitUInt,
    True,
    False,
}

impl From<TestResult> for bool {
    fn from(val: TestResult) -> Self {
        match val {
            TestResult::NotBitUInt | TestResult::False => false,
            TestResult::True => true,
        }
    }
}

pub fn miller_rabin_test<N>(number: N, rounds_count: Option<NonZeroU64>) -> TestResult
where
    N: ToBigUint,
{
    if let Some(number) = number.to_biguint() {
        if number.is_zero() || (&number % 2u8).is_zero() {
            return TestResult::False;
        }

        if let Ok(n) = u8::try_from(&number) {
            if PRIME_NUMBERS.contains(&n) {
                return TestResult::False;
            }
        }

        let mut s: u32 = 0;
        let mut nm1 = &number - 1u8;
        while !(&nm1 % 2u8).is_zero() {
            nm1 /= 2u8;
            s += 1;
        }
        let t = nm1;
        let mut rng = rand::thread_rng();
        for _ in 0..rounds_count.map(|k| k as u64).unwrap_or_else(|| log2(&number).unwrap()) {
            let num1: BigUint = rng.sample(RandomBits::new(192));
        }

        return TestResult::True;
    };

    return TestResult::NotBitUInt;
}

use std::num::NonZeroU64;

use num_bigint::BigUint;
use num_traits::{One, Zero};
use rand::{distributions::Uniform, Rng};
use rayon::prelude::*;

use crate::math::log2;

pub const FIRST_PRIME_NUMBERS: &[u8] = &[
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
    101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
    197, 199, 211, 223, 227, 229, 233, 239, 241, 251,
];

#[derive(Debug, PartialEq, Eq)]
pub enum TestResult {
    One,
    Composite,
    Prime,
    ProbablyPrime,
}

/// Performs the Miller-Rabin test
///
/// **number**       - the number to test
///
/// **rounds_count** - if None, then will be set to log2(**number**)
pub fn miller_rabin_test(number: &BigUint, rounds_count: Option<NonZeroU64>) -> TestResult {
    let const_2 = 2u8.into();
    if let Some(test_result) = small_pure_test(number, &const_2) {
        return test_result;
    }

    let mut s: u32 = 0;
    let nm1 = number - 1u8;
    let mut t = nm1.clone();
    while !(&t % 2u8).is_zero() {
        t /= 2u8;
        s += 1;
    }
    let mut rng = rand::thread_rng();
    let nm2 = &(number - 2u8);
    'A: for _ in 0..rounds_count
        .map(|k| k.into())
        .unwrap_or_else(|| log2(number).unwrap())
    {
        let rand_num: BigUint = rng.sample(Uniform::new(&const_2, nm2));
        let mut x = rand_num.modpow(&t, number);
        if x.is_one() || x == nm1 {
            continue;
        }
        for _ in 0..s {
            x = x.modpow(&const_2, number);
            if x.is_one() {
                return TestResult::Composite;
            }
            if x == nm1 {
                continue 'A;
            }
        }
        return TestResult::Composite;
    }

    TestResult::ProbablyPrime
}

/// Performs the Fermat test
///
/// **number**       - the number to test
///
/// **rounds_count** - if None, then will be set to log2(**number**)
pub fn fermat_test(number: &BigUint, rounds_count: Option<NonZeroU64>) -> TestResult {
    let const_2 = &2u8.into();
    if let Some(test_result) = small_pure_test(number, const_2) {
        return test_result;
    }

    let mut rng = rand::thread_rng();
    let nm1 = &(number - 1u8);
    let nm2 = &(number - 2u8);
    for _ in 0..rounds_count
        .map(|k| k.into())
        .unwrap_or_else(|| log2(number).unwrap())
    {
        let mut rand_num: BigUint = rng.sample(Uniform::new(const_2, nm2));
        while (&rand_num % number).is_zero() {
            rand_num = rng.sample(Uniform::new(const_2, nm2));
        }
        let x = rand_num.modpow(nm1, number);
        if !x.is_one() {
            return TestResult::Composite;
        }
    }

    TestResult::ProbablyPrime
}

fn small_pure_test(number: &BigUint, const_2: &BigUint) -> Option<TestResult> {
    if number.is_one() {
        return Some(TestResult::One);
    }
    if number == const_2 {
        return Some(TestResult::Prime);
    }
    if number.is_zero() || (number % 2u8).is_zero() {
        return Some(TestResult::Composite);
    }
    if let Ok(ref n) = u8::try_from(number) {
        if FIRST_PRIME_NUMBERS.into_par_iter().any(|p| p == n) {
            return Some(TestResult::Prime);
        }
        return Some(TestResult::Composite);
    }
    None
}

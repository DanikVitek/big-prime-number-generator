use crate::{
    math::log2,
    primality_test::{fermat_test, miller_rabin_test, TestResult},
};
use num_bigint::BigUint;

#[test]
fn _64_log2_equals_6() {
    assert_eq!(6u64, log2(&64u8.into()).unwrap())
}

#[test]
fn _100_log2_equals_6() {
    assert_eq!(6u64, log2(&100u8.into()).unwrap())
}

#[test]
fn miller_rabin_test_100_is_composite() {
    assert_eq!(
        TestResult::Composite,
        miller_rabin_test(&100u8.into(), None)
    )
}

#[test]
fn miller_rabin_test_100_000_is_composite() {
    assert_eq!(
        TestResult::Composite,
        miller_rabin_test(&BigUint::parse_bytes(b"100_000", 10).unwrap(), None)
    )
}

#[test]
fn miller_rabin_test_23_is_prime() {
    assert_eq!(TestResult::Prime, miller_rabin_test(&23u8.into(), None))
}

#[test]
fn miller_rabin_test_37_is_prime() {
    assert_eq!(TestResult::Prime, miller_rabin_test(&37u8.into(), None))
}

#[test]
fn miller_rabin_test_20_988_936_657_440_586_486_151_264_256_610_222_593_863_921_is_probably_prime()
{
    assert_eq!(
        TestResult::ProbablyPrime,
        miller_rabin_test(
            &BigUint::parse_bytes(
                b"20_988_936_657_440_586_486_151_264_256_610_222_593_863_921",
                10
            )
            .unwrap(),
            None
        )
    )
}

#[test]
#[allow(non_snake_case)]
fn miller_rabin_test_M607_is_probably_prime() {
    let M607 = BigUint::parse_bytes(
        b"531137992816767098689588206552468627329593117727031923199444138200403559860852242739162502265229285668889329486246501015346579337652707239409519978766587351943831270835393219031728127",
        10
    )
    .unwrap();
    assert_eq!(TestResult::ProbablyPrime, miller_rabin_test(&M607, None))
}

#[test]
fn fermat_test_100_is_composite() {
    assert_eq!(TestResult::Composite, fermat_test(&100u8.into(), None))
}

#[test]
fn fermat_test_100_000_is_composite() {
    assert_eq!(
        TestResult::Composite,
        fermat_test(&BigUint::parse_bytes(b"100_000", 10).unwrap(), None)
    )
}

#[test]
fn fermat_test_23_is_prime() {
    assert_eq!(TestResult::Prime, fermat_test(&23u8.into(), None))
}

#[test]
fn fermat_test_37_is_prime() {
    assert_eq!(TestResult::Prime, fermat_test(&37u8.into(), None))
}

#[test]
fn fermat_test_20_988_936_657_440_586_486_151_264_256_610_222_593_863_921_is_probably_prime() {
    assert_eq!(
        TestResult::ProbablyPrime,
        fermat_test(
            &BigUint::parse_bytes(
                b"20_988_936_657_440_586_486_151_264_256_610_222_593_863_921",
                10
            )
            .unwrap(),
            None
        )
    )
}

#[test]
#[allow(non_snake_case)]
fn fermat_test_M607_is_probably_prime() {
    let M607 = BigUint::parse_bytes(
        b"531137992816767098689588206552468627329593117727031923199444138200403559860852242739162502265229285668889329486246501015346579337652707239409519978766587351943831270835393219031728127",
        10
    )
    .unwrap();
    assert_eq!(TestResult::ProbablyPrime, fermat_test(&M607, None))
}

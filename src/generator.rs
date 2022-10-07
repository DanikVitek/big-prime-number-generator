use std::num::NonZeroU64;

use num_bigint::{BigUint, RandomBits};
use num_traits::Zero;
use rand::Rng;

use crate::primality_test::{fermat_test, miller_rabin_test};

pub fn generate_prime(n_bits: NonZeroU64) -> BigUint {
    let mut rng = rand::thread_rng();
    let ref distr = RandomBits::new(n_bits.get());

    loop {
        let mut prime_candidate: BigUint = rng.sample(distr);
        while (&prime_candidate % 2u8).is_zero() {
            prime_candidate = rng.sample(distr);
        }

        match miller_rabin_test(&prime_candidate, None) {
            crate::primality_test::TestResult::One
            | crate::primality_test::TestResult::Composite => continue,
            crate::primality_test::TestResult::Prime => return prime_candidate,
            crate::primality_test::TestResult::ProbablyPrime => {
                match fermat_test(&prime_candidate, None) {
                    crate::primality_test::TestResult::One
                    | crate::primality_test::TestResult::Composite => continue,
                    crate::primality_test::TestResult::Prime
                    | crate::primality_test::TestResult::ProbablyPrime => return prime_candidate,
                }
            }
        }
    }
}

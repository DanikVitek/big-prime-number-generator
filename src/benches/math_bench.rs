#[macro_use]
extern crate bencher;

use bencher::Bencher;
use num_bigint::BigUint;

fn log2(bench: &mut Bencher) {
    let ref n = BigUint::parse_bytes(
        b"20_988_936_657_440_586_486_151_264_256_610_222_593_863_921",
        10,
    )
    .unwrap();
    bench.iter(|| {
        for _ in 0..1000 {
            math::log2(n).unwrap();
        }
    })
}

fn log2_str(bench: &mut Bencher) {
    let ref n = BigUint::parse_bytes(
        b"20_988_936_657_440_586_486_151_264_256_610_222_593_863_921",
        10,
    )
    .unwrap();
    bench.iter(|| {
        for _ in 0..1000 {
            math::log2_str(n).unwrap();
        }
    })
}

benchmark_group!(benches, log2, log2_str);
benchmark_main!(benches);

mod math {
    use anyhow::anyhow;
    use num_bigint::BigUint;
    use num_traits::{One, Zero};

    pub fn log2(num: &BigUint) -> anyhow::Result<u64> {
        let mut num = num.clone();
        if num.is_zero() {
            return Err(anyhow!("Number must be positive"));
        }
        if num.is_one() {
            return Ok(0);
        }
        // Ok(num.to_str_radix(2).len() as u64 - 1)
        let mut log: u64 = 0;
        while !num.is_one() {
            num >>= 1;
            log += 1;
        }
        Ok(log)
    }

    pub fn log2_str(num: &BigUint) -> anyhow::Result<u64> {
        let num = num.clone();
        if num.is_zero() {
            return Err(anyhow!("Number must be positive"));
        }
        if num.is_one() {
            return Ok(0);
        }
        Ok(num.to_str_radix(2).len() as u64 - 1)
    }
}

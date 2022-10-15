mod generator;
mod math;
mod primality_test;
#[cfg(test)]
mod tests;

use crate::generator::generate_prime;
use anyhow::Context;
use indicatif::ParallelProgressIterator;
use inquire::{CustomType, InquireError};
use rayon::prelude::*;
use std::{fs::File, io::Write, num::NonZeroU64, sync::Mutex};

const OUTPUT_FILE: &str = "./output.txt";

fn main() -> anyhow::Result<()> {
    loop {
        match CustomType::<NonZeroU64>::new("Input the amount of number bits (> 0):")
            .prompt()
            .and_then(|n_bits| {
                CustomType::<NonZeroU64>::new("Input the amount of numbers to generate (> 0):")
                    .prompt()
                    .map(|amount| (n_bits, amount))
            }) {
            Ok((n_bits, amount)) => {
                let file = Mutex::new(File::create(OUTPUT_FILE).context("Creating output file")?);

                (0..amount.get())
                    .into_par_iter()
                    .progress_count(amount.get())
                    .for_each(|_| {
                        writeln!(
                            file.lock().expect("Locking file for thread-safe writing"),
                            "{}",
                            generate_prime(n_bits)
                        )
                        .context("Writing number to file")
                        .unwrap()
                    });
                println!("\n")
            }
            Err(e) => match e {
                InquireError::OperationCanceled | InquireError::OperationInterrupted => break,
                _ => {
                    println!("{e}");
                    break;
                }
            },
        }
    }

    Ok(())
}

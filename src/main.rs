use crate::smooths::Smooths;
use std::vec::Vec;
use once_cell::sync::Lazy;
use primal;

pub mod composite;
pub mod smooths;

// this should suffice for now
static PRIME_BOUND: usize = 1<<20;
static PRIMES: Lazy<Vec<usize>> = Lazy::new(|| primal::Sieve::new(PRIME_BOUND).primes_from(0).collect());

fn main() {
    let bound_log = 40;
    let mut smooths = Smooths::new(1<<bound_log);
    loop {
        smooths.add_prime();
        for i in 1..bound_log {
            let lower = 1<<i;
            let upper = 1<<(i+1);
            let l_ind = smooths.find_ind_le(lower).unwrap();
            let u_ind = smooths.find_ind_le(upper).unwrap();
            let mut mult_acc = 0u128;
            for ind in l_ind..u_ind {
                mult_acc += u128::from(smooths.get(ind).1);
            }
            println!("{}-smooth numbers with {i} bits have an average multiplicity of {}", PRIMES[smooths.primes-1], mult_acc/u128::try_from(u_ind-l_ind).unwrap());
        }
    }
}

use std::vec::Vec;
use crate::composite::Composite;
use super::{PRIMES};
use std::thread;
use rayon::prelude::*;

pub struct Smooths {
    // 1 < x <= upper_bound
    pub upper_bound: u64,
    pub primes: usize,
    smooths: Vec<(u64, u64)>,
}

impl Smooths {
    pub fn new(bound: u64) -> Self {
        Smooths{
            upper_bound: bound,
            primes: 0,
            smooths: vec![]
        }
    }

    pub fn len(&self) -> usize {
        self.smooths.len()
    }

    pub fn get(&self, ind: usize) -> (u64, u64) {
        self.smooths[ind]
    }

    pub fn ind(&self) -> usize {
        self.primes
    }

    pub fn add_prime(&mut self) {
        let mut smooths = self.init_gen(self.primes);
        self.smooths.append(&mut smooths);
        self.primes += 1;
        println!("Sorting all together");
        // sort in parallel
        self.smooths.par_sort_unstable();
        println!("Done adding prime");
    }

    fn init_gen(&self, ind: usize) -> Vec<(u64, u64)> {
        let prime = PRIMES[ind];
        let upper_bound = self.upper_bound;
        // generate all smooth numbers with a fixed exponent for the new prime
        let generate_with_fixed = |e_val: u32| {
            let mut c = Composite::new(ind, e_val);

            let mut smooths: Vec<(u64, u64)> = vec![];
            smooths.push((c.value, c.multiplicity()));
            // we break if the fixed exponent would change
            loop {
                c.inc_vec_with_bound(upper_bound);
                if c.es[ind] == e_val {
                    smooths.push((c.value, c.multiplicity()));
                } else {
                    break
                }
            }
            smooths
        };
        // for each possible exponent we start a thread
        let mut smooths = thread::scope(|s| {
            let mut handles = vec![];
            let p64: u64 = u64::try_from(prime).unwrap();
            let mut p: u64 = p64;
            let mut i = 1;
            loop {
                let h = s.spawn(move || generate_with_fixed(i));
                handles.push(h);
                i += 1;
                // to avoid overflow
                if u64::MAX/p64 < p {
                    break;
                }
                p *= p64;
                if p > upper_bound {
                    break;
                }
            }
            handles.into_iter().map(|h| h.join().unwrap()).collect::<Vec<Vec<(u64, u64)>>>().concat()
        });
        smooths.par_sort_unstable();
        println!("{}: Generated {} smooth numbers", prime, smooths.len());
        smooths
    }

    pub fn find_ind_gt(&self, b: u64) -> Option<usize> {
        let t = (b, u64::MAX);
        if self.smooths.len() == 0 || self.smooths[self.smooths.len()-1] <= t {
            return None;
        }
        let ind = match self.smooths.binary_search(&t) {
            Ok(x) => x+1,
            Err(x) => x,
        };
        assert!(self.smooths[ind] > t);
        Some(ind)
    }

    pub fn find_ind_le(&self, b: u64) -> Option<usize> {
        let t = (b, u64::MAX);
        if self.smooths.len() == 0 || self.smooths[0] > t {
            return None;
        }
        let ind = match self.smooths.binary_search(&t) {
            Ok(x) => x,
            Err(x) => x-1,
        };
        assert!(self.smooths[ind] <= t);
        Some(ind)
    }
}


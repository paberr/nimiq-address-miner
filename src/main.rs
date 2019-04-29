use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;

use nimiq_mnemonic::{Entropy, WORDLIST_EN};
use nimiq_mnemonic::key_derivation::ToExtendedPrivateKey;
use num_cpus;
use rand::RngCore;
use rand::rngs::OsRng;

use address::NimiqAddressString;
use args::Options;
use utils::add_to_entropy;
use std::time::Instant;

mod address;
mod args;
mod utils;

pub(crate) const DEFAULT_PATH: &'static str = "m/44'/242'/0'/0'";

fn main() {
    let mut args = match Options::parse() {
        Ok(mut args) => {
            if !args.strict {
                args.prefix = args.prefix.to_relaxed_form();
            }

            if !args.prefix.is_valid() {
                println!("Supplied prefix is invalid.");
                Options::create_app().print_help().unwrap();
                ::std::process::exit(0);
            }

            args
        },
        Err(e) => {
            println!("Error: {:?}", e);
            Options::create_app().print_help().unwrap();
            ::std::process::exit(0);
        },
    };

    // Relax prefix if necessary.
    if !args.strict {
        args.prefix = args.prefix.to_relaxed_form();
    }

    let num_threads = args.num_threads.unwrap_or_else(|| num_cpus::get());
    let mut threads = Vec::new();
    let solution_found  = Arc::new(AtomicBool::new(false));
    let mut csprng: OsRng = OsRng::new().unwrap();

    // Currently, we simply generate a new start entropy for every thread.
    for i in 0..num_threads {
        // Generate initial private key.
        let mut entropy = [0u8; 32];
        csprng.fill_bytes(&mut entropy);

        let entropy = Entropy::from(entropy);

        let thread_args = args.clone();
        let solution_found_flag = solution_found.clone();
        threads.push(thread::spawn(move || search_addresses(i, thread_args, entropy, solution_found_flag)));
    }

    // Join all threads.
    for thread in threads {
        thread.join().unwrap();
    }
}

fn search_addresses(thread_num: usize, args: Options, mut entropy: Entropy, solution_found: Arc<AtomicBool>) -> Option<Entropy> {
    // Try new private keys until we find a suitable match.
    let mut start = Instant::now();
    let mut rounds: usize = 0;
    loop {
        // Early return if other thread was faster.
        if solution_found.load(Ordering::Acquire) {
            return None;
        }

        // Stop early.
        if args.max_rounds.map(|max_rounds| rounds >= max_rounds).unwrap_or(false) {
            println!("Unfortunately, no matching address has been found.");
            break;
        }

        let mnemonic = entropy.to_mnemonic(WORDLIST_EN);
        let private_key = mnemonic.to_master_key(None).unwrap();
        let private_key = private_key.derive_path(&args.derivation_path).unwrap();
        let address = private_key.to_address();
        let mut address_str = address.to_user_friendly_address().replace(" ", "");

        if !args.strict {
            address_str = address_str.to_relaxed_form();
        }

        let matched = if args.contains {
            address_str.contains(&args.prefix)
        } else {
            // Allow matches only at beginning, after NQ, or after checksum.
            address_str.find(&args.prefix)
                .map(|index| index == 0 || index == 2 || index == 4)
                .unwrap_or(false)
        };

        // Display match and stop.
        if matched {
            solution_found.store(true, Ordering::Release);
            println!("Address found: {}", address.to_user_friendly_address());
            println!("24 words:");
            println!("{}", mnemonic);

            return Some(entropy);
        }

        rounds += 1;
        entropy = add_to_entropy(&entropy, 1);

        if args.verbose && rounds % 1000 == 0 {
            println!("Thread {}: {:?} per address, {} addresses processed", thread_num, start.elapsed() / 1000, rounds);
            start = Instant::now();
        }
    }

    None
}

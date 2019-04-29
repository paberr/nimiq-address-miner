use std::env;

use nimiq_mnemonic::{Entropy, WORDLIST_EN};
use nimiq_mnemonic::key_derivation::ToExtendedPrivateKey;
use rand::RngCore;
use rand::rngs::OsRng;

use address::NimiqAddressString;
use args::{ArgumentError, Arguments, print_help};
use utils::add_to_entropy;

mod address;
mod args;
mod utils;

const DEFAULT_PATH: &'static str = "m/44'/242'/0'/0'";

fn main() {
    let mut args = match Arguments::from_args(env::args()) {
        Ok(mut args) => {
            if !args.strict {
                args.prefix = args.prefix.to_relaxed_form();
            }

            if !args.prefix.is_valid() {
                print_help(Some(ArgumentError::InvalidPrefix));
                ::std::process::exit(0);
            }

            args
        },
        Err(e) => {
            print_help(Some(e));
            ::std::process::exit(0);
        },
    };

    // Generate initial private key.
    let mut entropy = [0u8; 32];
    let mut csprng: OsRng = OsRng::new().unwrap();
    csprng.fill_bytes(&mut entropy);

    let mut entropy = Entropy::from(entropy);

    // Relax prefix if necessary.
    if !args.strict {
        args.prefix = args.prefix.to_relaxed_form();
    }

    // Try new private keys until we find a suitable match.
    let mut rounds: usize = 0;
    loop {
        // Stop early.
        if args.max_rounds.map(|max_rounds| rounds >= max_rounds).unwrap_or(false) {
            println!("Unfortunately, no matching address has been found.");
            break;
        }

        let mnemonic = entropy.to_mnemonic(WORDLIST_EN);
        let private_key = mnemonic.to_master_key(None).unwrap();
        let private_key = private_key.derive_path(DEFAULT_PATH).unwrap();
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
            println!("Address found: {}", address.to_user_friendly_address());
            println!("24 words:");
            println!("{}", mnemonic);
            break;
        }

        rounds += 1;
        entropy = add_to_entropy(&entropy, 1);
    }
}

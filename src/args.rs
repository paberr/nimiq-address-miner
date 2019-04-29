use std::env::Args;
use std::str::FromStr;

pub(crate) struct Arguments {
    pub(crate) strict: bool,
    pub(crate) contains: bool,
    pub(crate) prefix: String,
    pub(crate) max_rounds: Option<usize>,
}

#[derive(Clone, Debug)]
pub(crate) enum ArgumentError {
    PrefixMissing,
    InvalidPrefix,
}

impl Arguments {
    pub(crate) fn from_args(args: Args) -> Result<Arguments, ArgumentError> {
        let mut args: Vec<String> = args.collect();

        if args.len() < 2 {
            return Err(ArgumentError::PrefixMissing);
        }

        // Remove first argument.
        args.remove(0);

        let mut prefix: Option<String> = None;
        let mut max_rounds: Option<usize> = None;
        let mut strict = false;
        let mut contains = false;

        for arg in args {
            if arg == "--strict" {
                strict = true;
            } else if arg == "--contains" {
                contains = true;
            } else if prefix.is_none() {
                // Read prefix.
                prefix = Some(arg.to_ascii_uppercase());
            } else if max_rounds.is_none() {
                // Try parsing as usize.
                max_rounds = usize::from_str(&arg).ok();
            } else {
                // No more arguments to be processed.
                break;
            }
        }

        let prefix = match prefix {
            None => return Err(ArgumentError::PrefixMissing),
            Some(prefix) => prefix,
        };

        Ok(Arguments {
            strict,
            contains,
            prefix,
            max_rounds,
        })
    }
}

pub(crate) fn print_help(error: Option<ArgumentError>) {
    if let Some(e) = error {
        println!("{:?}", e);
    }
    println!("Usage: vanity_generate <prefix> [<max_rounds>] [--strict] [--contains]");
    println!("  --strict    If enabled, characters will not be mapped to similar looking characters.");
    println!("  --contains  If enabled, the prefix may also be contained in the middle of the address.");
}
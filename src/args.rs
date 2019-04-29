use std::str::FromStr;
use clap::{Arg, App};
use crate::DEFAULT_PATH;

#[derive(Clone)]
pub(crate) struct Options {
    pub(crate) strict: bool,
    pub(crate) contains: bool,
    pub(crate) verbose: bool,
    pub(crate) prefix: String,
    pub(crate) derivation_path: String,
    pub(crate) max_rounds: Option<usize>,
    pub(crate) num_threads: Option<usize>,
}

#[derive(Clone, Debug)]
pub(crate) enum ArgumentError {
    PrefixMissing,
    InvalidRounds,
    InvalidThreads,
}

impl Options {
    pub(crate) fn create_app<'a, 'b>() -> App<'a, 'b> {
        App::new("vanity_generate")
            .version("0.1.0")
            .about("Nimiq Vanity Address Generator")
            .arg(Arg::with_name("prefix")
                .value_name("PREFIX")
                .help("Address prefix to look for.")
                .takes_value(true)
                .required(true))
            .arg(Arg::with_name("max_rounds")
                .long("rounds")
                .value_name("MAX_ROUNDS")
                .help("Maximum number of rounds to run per thread.")
                .takes_value(true))
            .arg(Arg::with_name("num_threads")
                .short("t")
                .long("threads")
                .value_name("THREADS")
                .help("Number of threads to use.")
                .takes_value(true))
            .arg(Arg::with_name("derivation_path")
                .long("derivation-path")
                .value_name("PATH")
                .help("A custom key derivation path to use.")
                .takes_value(true)
                .default_value(DEFAULT_PATH))
            .arg(Arg::with_name("strict")
                .long("strict")
                .help("If enabled, characters will not be mapped to similar looking characters.")
                .takes_value(false))
            .arg(Arg::with_name("contains")
                .long("contains")
                .help("If enabled, the prefix may also be contained in the middle of the address.")
                .takes_value(false))
            .arg(Arg::with_name("verbose")
                .short("v")
                .help("Display statistics about speed every 1000 addresses.")
                .takes_value(false))
    }

    /// Parses a command line option from a string into `T` and returns `error`, when parsing fails.
    fn parse_option<T: FromStr>(value: Option<&str>, error: ArgumentError) -> Result<Option<T>, ArgumentError> {
        match value {
            None => Ok(None),
            Some(s) => match T::from_str(s.trim()) {
                Err(_) => Err(error), // type of _: <T as FromStr>::Err
                Ok(v) => Ok(Some(v))
            }
        }
    }

    fn parse_option_string(value: Option<&str>) -> Option<String> {
        value.map(String::from)
    }

    pub(crate) fn parse() -> Result<Options, ArgumentError> {
        let app = Self::create_app();
        let matches = app.get_matches();

        Ok(Options {
            strict: matches.is_present("strict"),
            contains: matches.is_present("contains"),
            verbose: matches.is_present("verbose"),
            prefix: Self::parse_option_string(matches.value_of("prefix"))
                .map(|s| s.to_uppercase())
                .ok_or(ArgumentError::PrefixMissing)?,
            derivation_path: Self::parse_option_string(matches.value_of("derivation_path")).unwrap(), // Has a default value.
            max_rounds: Self::parse_option(matches.value_of("max_rounds"), ArgumentError::InvalidRounds)?,
            num_threads: Self::parse_option(matches.value_of("num_threads"), ArgumentError::InvalidThreads)?,
        })
    }
}

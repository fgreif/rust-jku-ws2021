use core::panic;
use std::{io, str::ParseBoolError};
use rand::{thread_rng, Rng};
use std::num::ParseIntError;
use clap::{Arg, ArgMatches, App};

// PLEASE NOTE: must include clap = "3.0.10" in Cargo.toml


// SECTION: ERROR HANDLING IMPLEMENTATION

#[derive(Debug)]
enum ArgsError {
    ParseError(ParseIntError),
}

#[derive(Debug)]
enum RangeError {
    ParseError(ParseBoolError),
}

impl std::fmt::Display for ArgsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArgsError::ParseError(err) => write!(f, "ParseError: Not a valid Integer: {}", err),
        }
    }
}

impl std::fmt::Display for RangeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RangeError::ParseError(err) => write!(f, "ParseError: Not a valid Range: {}", err),
        }
    }
}

impl std::error::Error for ArgsError {}
impl std::error::Error for RangeError {}

impl From<ParseIntError> for ArgsError {
    fn from(v: ParseIntError) -> Self {
        ArgsError::ParseError(v)
    }
}

impl From<ParseBoolError> for RangeError {
    fn from(v: ParseBoolError) -> Self {
        RangeError::ParseError(v)
    }
}

fn check_range(lower: u32, upper: u32) -> Result<bool, RangeError> {
    Ok(lower < upper)
}

fn check_for_int(num: &str) -> Result<u32, ArgsError> {
    let num: u32 = num.trim().parse()?;
    Ok(num)
}

// checks if arg exists
// checks if arg is integer
fn check_args(args: &ArgMatches, arg: &str) {
    if args.is_present(arg) {
        match check_for_int(args.value_of(arg).unwrap()) {
            Ok(result) => println!("{} is a valid Integer", result),
            Err(ArgsError::ParseError(err)) => {
                eprintln!("Error when parsing the argument {}: {}", arg, err);
                panic!("Abort program!");
            },
        }
    }
}

// checks if range is within bounds (lower < upper must hold)
fn check_bounds(lower: u32, upper: u32) {
    match check_range(lower, upper) {
        Ok(result) => println!("The range is {} (lower < upper)", result),
        Err(RangeError::ParseError(err)) => {
            eprintln!("Error: condition lower < upper is {}", err);
            panic!("Abort Program");
        }
    }
}

// handles user input
fn input() -> u32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Not a valid String");
    let input_parsed: u32 = input.trim().parse().expect("Not a valid Integer");
    return input_parsed;
}

// USAGE: 
// cargo build
// cargo run -- --help   --> shows which parameters are possible
// cargo run -- --tries x --lower y --upper z  --> x, y, z are integer numbers for
// number of tries, lower bound, upper bound  --> optional parameters!
// default values: tries=inf, lower=0, upper=u32::MAX
fn main() {
    let args = App::new("Guess the number!")
        .version("1.0")
        .author("Florian Greif k01455312")
        .about("Guess the number game!")
        .arg(Arg::new("tries")
                .short('t')
                .long("tries")
                .takes_value(true)
                .help("Limit number of tries (default: infinite)"))
        .arg(Arg::new("lower_bound")
                .short('l')
                .long("lower")
                .takes_value(true)
                .help("Lower bound of range (default: 0)"))
        .arg(Arg::new("upper_bound")
                .short('u')
                .long("upper")
                .takes_value(true)
                .help("upper bound of range (default: u32::MAX)"))
        .get_matches();

    //First check for correct argument types, if one is incorrect -> panic!

    check_args(&args, "tries");
    check_args(&args, "lower_bound");
    check_args(&args, "upper_bound");

    //if the argument types are correct, parse them
    let tries = args.value_of_t("tries").unwrap_or(0);
    let lower = args.value_of_t("lower_bound").unwrap_or(0);
    let upper = args.value_of_t("upper_bound").unwrap_or(std::u32::MAX);

    check_bounds(lower, upper);
    let num_to_guess: u32 = thread_rng().gen_range(lower..upper);

    //uncomment for debugging purposes
    //println!("GENERATED RANDOM NUM: {} ", num_to_guess);

    let mut tries_counter: usize = 0;

    //game loop
    loop {
        println!("Enter a number!");
        let input = input();
        tries_counter += 1;

        let output = match input.cmp(&num_to_guess) {
            std::cmp::Ordering::Less => "Too low!",
            std::cmp::Ordering::Greater => "Too high!",
            std::cmp::Ordering::Equal => {
                println!("Congrats! Found the number after {} tries!", tries_counter);
                break;
            }
        };

        println!("{}", output);
        if tries != 0 && tries == tries_counter {
            println!("Maximum number of tries reached, abort!");
            break;
        }
    }
}

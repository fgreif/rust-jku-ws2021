/**
 * Florian Greif, Matr.Nr. 01455312
 */

use std::env;
use rand::{thread_rng, Rng};

struct Lotto {
    take: usize,
    from: usize,
    numbers: Vec<usize>,
}

impl Lotto {
    fn new(take: usize, from: usize) -> Self {
        let mut numbers: Vec<usize> = Vec::new();

        // add numbers until len() == take, avoid adding similar numbers
        while numbers.len() != take {
            let num: usize = thread_rng().gen_range(0..from+1); // upper bound is exclusive, hence the +1
            if !numbers.contains(&num) {
                numbers.push(num);
            }
        }
        let lotto: Lotto = Lotto {
            take,
            from,
            numbers,
        };
        return lotto;
    }
}

fn format_lotto_results(lotto: &Lotto) -> String {
    format!("{} of {}: {:?}", lotto.take, lotto.from, lotto.numbers)
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // check for correct params
    // must be uneven (but > 1) since args[0] = program and the games 
    // are in pairs of numbers according to the task, so 2n+1 with n being the number of games
    // eg.: 6 45 7 69 --> 2 games, 4 arguments, but args.len() == 5
    let running_condition: bool = args.len() > 1 && args.len() % 2 == 1 && !args_contain_negative(&args);

    match running_condition {
        true => run(args),
        false => args_error()
    };
}

fn run(args: Vec<String>) {
    let mut lotto_vec: Vec<Lotto> = Vec::new();
    for game in (1..args.len()).step_by(2) {
        let take: usize = args[game].parse().unwrap();
        let from: usize = args[game+1].parse().unwrap();

        // error handling (does not make sense to play for example 10 of 2)
        if take > from {
            args_error();
        }

        let lotto: Lotto = Lotto::new(take, from);
        lotto_vec.push(lotto);
    }
    for lotto in lotto_vec {
        println!("{}", format_lotto_results(&lotto));
    }
}

fn args_contain_negative(args: &Vec<String>) -> bool {
    // skip first arg
    for n in 1..args.len()-1 {
        if args[n].parse::<i32>().unwrap() < 1 {
            return true;          
        }
    }
    return false;
}

fn args_error() {
    println!("ERROR: invalid arguments");
}


// just a small tribute to a great TV show :)
#[test]
fn test_lost_nums() {
    let lotto1 = Lotto::new(4, 8);
    let lotto2 = Lotto::new(15, 16);
    let lotto3 = Lotto::new(23, 42);

    assert_eq!(lotto1.numbers.len(), 4 );
    assert_eq!(lotto2.numbers.len(), 15 );
    assert_eq!(lotto3.numbers.len(), 23);
}

#[test]
fn test_format_lotto_results() {
    let lotto = Lotto {
        take: 6,
        from: 45,
        numbers: vec![2, 3, 10, 25, 30, 40],
    };

    assert_eq!(
        "6 of 45: [2, 3, 10, 25, 30, 40]",
        format_lotto_results(&lotto)
    );
}

#[test]
fn test_lotto_constructor() {
    let lotto = Lotto::new(6, 45);

    let numbers = lotto.numbers;

    assert_eq!(numbers.len(), 6);
}

#[test]
fn test_lotto_constructor_uniques() {
    use std::collections::HashSet;
    let lotto = Lotto::new(6, 45);

    let numbers = lotto.numbers;
    let set: HashSet<usize> = numbers.into_iter().collect();

    assert_eq!(set.len(), 6);
}

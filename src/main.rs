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

        while numbers.len() != take {
            let num: usize = thread_rng().gen_range(0..from + 1);
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
    // Tip: Use the format macro
    format!("{} of {}: {:?}", lotto.take, lotto.from, lotto.numbers)
}

fn main() {
    // working program including the bonus:

    let args: Vec<String> = env::args().collect();
    let mut lotto_vec: Vec<Lotto> = Vec::new();

    // check for correct number of params
    // must be uneven since args[0] = program name and games are in pairs, so 2n+1
    if args.len() % 2 == 1 {
        for game in (1..args.len()).step_by(2) {
            let take: usize = args[game].parse().unwrap();
            let from: usize = args[game + 1].parse().unwrap();
            let lotto: Lotto = Lotto::new(take, from);
            lotto_vec.push(lotto);
        }
        for lotto in lotto_vec {
            println!("{}", format_lotto_results(&lotto));
        }
    } else {
        println!("ERROR: incorrect number of arguments, must be 2 or a multiple of 2");
    }
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

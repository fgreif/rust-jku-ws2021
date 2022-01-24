# The Rust Programming Language in a Day: Task #1 - The Lottery

## Task description

Write a command line tool that takes a list of numbers for international lotteries, eg. 6 out of 45, 6 out of 49, 5 out of 50, etc.

- `cargo run -- 6 45` means to take 6 out of 45
- This will yield 6 random numbers out of a pot of 45 numbers
- Write a struct `Lotto` representing your Lotto results
- Write a function `format_lotto_results` that formats your lotto results for pretty printing
- Make sure you adhere ownership and borrowing rules
- *Bonus*: Allow for multiple runs. E.g. `cargo run -- 6 45 6 49 5 50` does three runs: 6 out of 45, 6 out of 49, 5 out of 50
- Use the `rand` crate for random number generation ([this might be helpful](https://docs.rs/rand/0.6.4/rand/seq/trait.SliceRandom.html#tymethod.choose_multiple)) 


## Working with Codespaces

1. Fork this repo
2. On the upper right corner, there is a button named *Code*
3. Click the *Code* button and click "New codespace"
4. A new codespace for you pops up. You can reuse this codespace after creation.

You are all set up with VSCode, Rust, a Terminal, etc.

See more about [Codespaces](GitHub Codespaces)

## Tests and execution

1. Run the tests with `cargo test`. All tests need to pass!

2. Run the app with `cargo run -- 6 45`. This should be the desired output

```
6 of 45: [31, 29, 38, 4, 21, 24]
```

*With the numbers being different each time of course*!

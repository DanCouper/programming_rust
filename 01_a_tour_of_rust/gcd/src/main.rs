/// The `use` declarations bring the `Write` and `FromStr`
/// [traits](https://doc.rust-lang.org/book/second-edition/ch10-02-traits.html) into scope.
/// 
/// A trait, basically, is a way to group a set of method signatures
/// together under a namespace. Elixir has behaviours that do the same
/// thing.
/// 
/// In this case, `Write` has a `write_fmt` method that writes to a stream.
/// The macro `writeln!` (mote macros are denoted by the `!` suffix) expands
/// to code that uses this. Similarly, `u64` implements the `from_str`
/// method defined in `FromStr`.
use std::io::Write;
use std::str::FromStr;

/// `main` does not return anything, so there is no need for the
/// return value to be specified with `->`.
/// 
/// 1. `Vec` is Rust's growable vector type - analogous to JS arrays.
///    Still needs to be mutable to allow items to be added to it.
///    Because u64's are being pushed into it, Rust can infer the type.
/// 2. Process the command-line arguments by looping over them.
///    `std::env::args` returns an iterator. The first value it returns
///    is the name of the program being run, so skip is used to ignore that.
/// 3. `from_str` returns a result type (`Ok(v)`/`Err(e)`). `Result`'s
///     `expect` method - this prints a method on Err, otherwise returns v. Nice!
/// 4. If there are no numbers, need to exit. `stderr` also returns a result,
///    and could use `expect` again, but `unwrap` will do here.
/// 5. The meat and potatoes. `d` is used to store the running total, simple enough.
///    the `&` and `*` though:
///     - `&` _borrows_ a reference to the vectors elements from the second
///        onwards. The for loop borrows each element in turn.
///     - `*` _dereferences_ the m, yielding the value.
///     - Finally, when the loop completes, `numbers`, which owned the
///       vector, frees it when it goes out of scope.
/// 6. The println! macro does it's string substitution thing. Rust
///    assumes that if `main` returns, the program is complete, so
///    there is no need to explicitly return 0.
fn main() {
    // 1
    let mut numbers = Vec::new();
    // 2
    for arg in std::env::args().skip(1) {
        // 3
        numbers.push(u64::from_str(&arg).expect("error parsing argument"));
    }
    // 4
    if numbers.len() == 0 {
        writeln!(std::io::stderr(), "Usage: gcd NUMBER ...").unwrap();
        std::process::exit(1);
    }
    // 5
    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }
    // 6
    println!("The greated common divisor of {:?} is {}", numbers, d);
}

/// Pretty obvious first function.
/// Types are defined after the parameter, with the return type defined
/// the `->`. `mut` is used to denote that a variable can be mutated.
/// `let` is used for local variables. `assert!` checks for preconditions,
/// and exits if the assertion fails (it causes a _panic_).
/// NOTE there is a `debug_assert!` which allows the assertion to be
/// skipped it the program is compiled for speed.
fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

/// The `#[test]` is an example of an attribute: this allows functions
/// to be marked with certain meta information that the compiler can understand.
#[test]
fn test_gcd() {
    assert_eq!(gcd(14, 15), 1);

    assert_eq!(gcd(2 * 3 * 5 * 11 * 17, 3 * 7 * 11 * 13 * 19), 3 * 11);
}

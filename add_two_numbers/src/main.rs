#[macro_use]
extern crate clap;

use clap::Clap;

#[derive(Clap, Debug)]
#[clap(name = "rusty")]
/// random app for demoing rust features
struct Opts {
    #[clap(required = true, min_values = 2, max_values = 2)]
    numbers: Vec<i32>,
}

/// # Add Two Numbers
/// 
/// A pretty rad function that takes two numbers and adds them together to 
/// return a third number. So many uses... so little time.
///
/// ## Arguments
///
/// * `first`: The first of the two numbers to add together
/// * `second`: The second of the two numbers to add together

fn add_two_numbers(first: i32, second: i32) -> i32 {
    first + second
}


/// # Main
///
/// Typical main function like you'd find in any language. This is where the 
/// binary application runs from!

fn main() {
    let mut opts = Opts::parse();

    println!(
        "Result: {:?}", add_two_numbers(
            opts.numbers.pop().expect("Not enough numbers"), 
            opts.numbers.pop().expect("Not enough numbers")
        )
    );
}

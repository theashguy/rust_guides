fn write_a_string(thing_to_write: String) {
    println!("{}", thing_to_write);
}

fn main() {
    let mut thing = "My Stringy Thing".to_string();

    write_a_string(thing);

    // thing = format!("Some kind of mutation to {}", thing);
}

// --
// 1. Why Rust is magic
// 2. The first thing that makes people think Rust is dumb
// 3. The thing people find hardest about Rust-- Lifetimes
// 4. While you're learning Overallocate!
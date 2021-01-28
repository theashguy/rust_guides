#[derive(Debug)]
enum Numberses {
    Unsigned(u32),
    Integer(i32),
    Float(f32)
}

fn output_a_number(number_opt: Option<Numberses>) {
    match number_opt {
        Some(number) =>{
            match number {
                Numberses::Float(number) => println!("My Floaty Number is {:?}", number),
                Numberses::Integer(number) => println!("My Integer Number is {:?}", number),
                _ => println!("I don't understand!"),
            }
        }, 
        None => println!("Nothing was passed!")
    }
}

fn main () {
    // Each of these has a 8, 16, 64 variant
    // let number: u8 = -100; 
    let number = None;
    // let number: f32 = 100.0; // Floating point numbers need the decimal

    output_a_number(number)
}

// ---

// 1. Go through basic numbers cases

// 2. Build a numbers struct


// 3. Match on the numbers struct
// 4. Talk about Option<Something>
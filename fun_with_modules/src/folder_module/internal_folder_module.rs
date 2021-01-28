pub fn print_something () {
    println!("Something from the internal folder module")
}

pub mod first_internal_internal_folder_module {
    pub fn print_something () {
        println!("Something from the first module")
    }
}

pub mod second_internal_internal_folder_module {
    pub fn print_something () {
        println!("Something from the second module")
    }
}
#[macro_use]
extern crate serde_json;

use serde::{Deserialize, Serialize};
use serde_json::Result;

// --
// Some structures

enum Country {
    Australia, 
    NewZealand, 
    UnitedStates,
}

enum Job {
    Cashier, 
    Pilot, 
    VicePresident(Country),
    President(Country),
}

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
    job: Option<Job>,
}

impl Person {
    fn holla(self) { println!("Pumped!") }
}

fn run_thing(person: Person) {
    person.holla()
}

fn main() {
    // let joe = Person {
    //     name: "Joe Robinette".to_string(),
    //     age: 99,
    //     job: Job::VicePresident(Country::UnitedStates)
    // };

    let random_json = r#"{
        name: 'Joe Robinette',
        age: 99
    }"#;

    let joe: Person = serde_json::from_str(random_json).expect("This will work");

    run_thing(joe);
}

// --
// 1. The Rust thing that looks like objects
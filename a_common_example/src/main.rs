#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/<what>")]
fn index(what: Option<String>) -> String {
    match what {
        Some(what) => format!("Hello, {}", what),
        None => "Hello Worlde!".to_string(),
    }
}

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}

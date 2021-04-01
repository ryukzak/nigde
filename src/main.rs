#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

// http://localhost:8000/hello/John/58
#[get("/hello/<name>/<age>")]
fn hello(name: String, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}

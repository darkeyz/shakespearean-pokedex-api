#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

use rocket::response::content;

mod pokedex;
#[cfg(test)] mod tests;
use crate::pokedex::routes;
use crate::pokedex::types;
use crate::pokedex::actions;

#[catch(400)]
fn bad_request() -> content::Html<String> {
    content::Html(format!("<p>Sorry :( Pokemon not found! Are you sure this pokemon exists?</p>"))
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().register(catchers![bad_request]).mount("/", routes![routes::get_pokemon])
}

fn main() {
    rocket().launch();
}
#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/pokemon/<pokemon_name>")]
fn get_pokemon(pokemon_name: String) -> String {
    format!("Hello, i'm {}!", pokemon_name)
}

fn main() {
    rocket::ignite().mount("/", routes![get_pokemon]).launch();
}
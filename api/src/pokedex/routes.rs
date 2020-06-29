use rocket_contrib::json::Json;
use rocket::http::Status;


use crate::actions::get_pokemon_from_external_api;
use crate::actions::translate_description;

#[derive(Serialize, Deserialize, Debug)]
pub struct Output {
    name: String,
    description: String
}

#[get("/pokemon/<pokemon_name>")]
pub fn get_pokemon(pokemon_name: String) -> Result<Json<Output>, Status> {
    let (pokemon, status) = get_pokemon_from_external_api(pokemon_name).unwrap();
    match status.as_u16() {
        200 => {
            let pokemon = pokemon.unwrap();
            let (shakespearean_description, status) = translate_description(pokemon.info.description).unwrap();
            println!("{:?}", shakespearean_description);
            match status.as_u16() {
                200 => return Ok(Json(Output { name: pokemon.name, description: shakespearean_description.unwrap().contents.translated })),
                400 => return Err(Status::BadRequest),
                429 => return Err(Status::TooManyRequests),
                _ => return Err(Status::InternalServerError),
            }
        },
        400 => return Err(Status::BadRequest),
        _ => return Err(Status::InternalServerError),
    }
}
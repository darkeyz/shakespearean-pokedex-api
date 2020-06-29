use reqwest::Error;
use reqwest::StatusCode;
use crate::types::Pokemon;
use crate::types::ShakespeareanDescription;

pub fn get_pokemon_from_external_api(pokemon_name: String) -> Result<(Result<Pokemon, Error>, StatusCode), Error> {
    let external_api_url = "https://courses.cs.washington.edu/courses/cse154/webservices/pokedex/pokedex.php?pokemon=";
    let url = format!("{}{}", external_api_url, pokemon_name);
    let response = reqwest::blocking::get(&url)?;
    let status = response.status();
    let pokemon = response.json::<Pokemon>();
    Ok((pokemon, status))
}

pub fn translate_description(description: String) -> Result<(Result<ShakespeareanDescription, Error>, StatusCode), Error> {
    let external_api_url = "https://api.funtranslations.com/translate/shakespeare.json?text=";
    let url = format!("{}{}", external_api_url, description);
    let response = reqwest::blocking::get(&url)?;
    let status = response.status();
    let description = response.json::<ShakespeareanDescription>();
    Ok((description, status))
}
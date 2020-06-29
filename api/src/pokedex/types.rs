#[derive(Serialize, Deserialize, Debug)]
pub struct Pokemon {
    pub name: String,
    pub info: PokemonInfo
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PokemonInfo {
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ShakespeareanDescription {
    success: ShakespeareanSuccess,
    pub contents: ShakespeareanContents
}

#[derive(Serialize, Deserialize, Debug)]
struct ShakespeareanSuccess { total: i32 }

#[derive(Serialize, Deserialize, Debug)]
pub struct ShakespeareanContents {
    pub translated: String,
    translation: String,
    text: String
}
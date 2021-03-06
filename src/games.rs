use {Category, DateTime, Parse};
use serde_json::from_str;

#[derive(Serialize, Deserialize)]
pub struct Games {
    games: Vec<Game>,
}

impl Parse<Vec<Game>> for Games {
    fn parse(raw: &str) -> Result<Vec<Game>, ()> {
        let v: Games = from_str(raw).unwrap();
        Ok(v.games)
    }
}

#[derive(Serialize, Deserialize)]
struct GameObject {
    game: Game,
}

#[derive(Serialize, Deserialize)]
pub struct Game {
    id: u32,
    name: String,
    short_name: Option<String>,
    created_at: DateTime,
    updated_at: DateTime,
    #[serde(default)]
    categories: Option<Vec<Category>>,
}

impl Parse<Game> for Game {
    fn parse(raw: &str) -> Result<Game, ()> {
        let v: GameObject = from_str(raw).unwrap();
        Ok(v.game)
    }
}

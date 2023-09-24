use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Movie {
    pub id: u32,
    pub title: String,
    pub year: String,
    pub runtime: String,
    pub genres: Vec<Genre>,
    pub director: String,
    pub actors: String,
    pub plot: String,
    #[serde(alias = "posterUrl")]
    pub poster_url: String,
}

pub type Genre = String;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MoviesData {
    pub genres: Vec<Genre>,
    pub movies: Vec<Movie>,
}

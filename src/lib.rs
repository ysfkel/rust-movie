mod types;
use rand::prelude::*;
use serde;
use std::collections::HashMap;
use std::{fs, io};
pub use types::{Genre, Movie, MoviesData};

pub fn get_filtered_movies(genres: Vec<types::Genre>) -> Vec<types::Movie> {
    let movies = get_movies();

    // select random movie if genre is empty
    if genres.len() == 0 {
        if let Some(m) = movies.iter().choose(&mut thread_rng()) {
            return vec![m.clone()];
        } else {
            return vec![];
        }
    }

    let lookup = get_genres(genres); // creates a hashmap for genre
    let mut filtered_movies: Vec<types::Movie> = Vec::new(); // use to hold our final result
    let mut index = 0; // each time we match a genre ,

    // loop through movies and add movies that match provided genres to the filtered_movies vec
    for m in movies.iter() {
        let mut genre_match_count: u32 = 0;
        for (i, g) in m.genres.iter().enumerate() {
            if lookup.contains_key(g) == false {
                break;
            } else {
                // if we have compared all the genres of this movie, we add the movie to filtered_movies
                if i == m.genres.len() - 1 {
                    filtered_movies.insert(index, m.clone());
                    index += 1;
                }
            }
        }
    }

    // sort filtered movies in descending order by the count of genre
    filtered_movies.sort_by(|x, y| y.genres.len().cmp(&x.genres.len()));

    filtered_movies
}

fn get_movies() -> Vec<types::Movie> {
    let mut movies_data: MoviesData = {
        let data = fs::read_to_string("src/db.json").expect("error reading file");
        serde_json::from_str(&data).unwrap()
    };
    movies_data.movies
}

fn get_genres(genres: Vec<String>) -> HashMap<String, bool> {
    let mut _genres = HashMap::new();

    for i in genres.iter() {
        _genres.insert(i.clone(), true);
    }
    _genres
}

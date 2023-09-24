mod types;
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs;
pub use types::{Genre, Movie, MoviesData};

/**
 * The time complexity of the functions are
 * 1. get_filtered_movies - The overall time complexity of get_filtered_movies is O(n * m) where n the number of movies and
 *    g is the average number of genres per movie . The other operations (explained below)have lower time complexity
 * 2. get_movies - Reads movies json data from disk: The time complexity to read the json file is O(n)
 *   where is the size of the json file.
 * 3. get_genres - creates a hashmap of movie genres . The time complexity is O(g) where g is the number of genres
 * 4. filtered_movies the sort function used to sort the filtered_movies vec has a time complexity of O(n * log(n))
 *    where k is the number of elements in the vector.
 * 5. get_random_index - generates a random number based on the current system time
 *     has a time complexity of O(1) because it involves basic arithmetic operations
 */

pub fn get_filtered_movies(genres: Vec<types::Genre>) -> Vec<types::Movie> {
    let movies = get_movies();

    if movies.len() == 0 {
        return vec![];
    }

    // select random movie if genre is empty
    if genres.len() == 0 {
        let random_index = get_random_index(movies.len());
        return vec![movies[random_index].clone()];
    }

    let lookup = get_genres(genres); // creates a hashmap for genre
    let mut filtered_movies: Vec<types::Movie> = Vec::new(); // use to hold our final result
    let mut index = 0; // each time we match a genre ,

    // loop through movies and add movies that match provided genres to the filtered_movies vec
    for m in movies.iter() {
        for (i, g) in m.genres.iter().enumerate() {
            if lookup.contains_key(g) == false {
                break;
            } else if i == m.genres.len() - 1 { // if we have compared all the genres of this movie, we add the movie to filtered_movies
                filtered_movies.insert(index, m.clone());
                index += 1;
            }
        }
    }

    // sort filtered movies in descending order by the length of genres
    filtered_movies.sort_by(|x, y| y.genres.len().cmp(&x.genres.len()));

    filtered_movies
}

fn get_movies() -> Vec<types::Movie> {
    let  movies_data: MoviesData = {
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

fn get_random_index(vector_length: usize) -> usize {
    // Get the current system time since the Unix epoch
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_nanos() as u64;

    // Use the timestamp to calculate a random index within the vector's bounds
    let random_index = (timestamp % vector_length as u64) as usize;

    random_index
}

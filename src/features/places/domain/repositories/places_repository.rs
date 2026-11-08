use std::path::PathBuf;

use crate::features::places::domain::entities::place::Place;

pub trait PlacesRepository {
    fn places(&self) -> Vec<Place>;
    fn home(&self) -> Option<PathBuf>;
}

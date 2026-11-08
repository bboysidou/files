use crate::features::places::data::repositories::xdg_places_repository::XdgPlacesRepository;
use crate::features::places::domain::repositories::places_repository::PlacesRepository;

pub fn places_repository() -> Box<dyn PlacesRepository> {
    Box::new(XdgPlacesRepository)
}

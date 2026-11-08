use std::path::PathBuf;

use crate::core::enums::places::place_kind::PlaceKind;
use crate::features::places::domain::entities::place::{Place, PlaceSection, PlaceTarget};
use crate::features::places::domain::repositories::places_repository::PlacesRepository;

const FILESYSTEM_ROOT: &str = "/";

type DirectoryResolver = fn() -> Option<PathBuf>;

const USER_DIRECTORIES: [(PlaceKind, DirectoryResolver); 7] = [
    (PlaceKind::Home, dirs::home_dir),
    (PlaceKind::Desktop, dirs::desktop_dir),
    (PlaceKind::Documents, dirs::document_dir),
    (PlaceKind::Downloads, dirs::download_dir),
    (PlaceKind::Pictures, dirs::picture_dir),
    (PlaceKind::Music, dirs::audio_dir),
    (PlaceKind::Videos, dirs::video_dir),
];

pub struct XdgPlacesRepository;

impl XdgPlacesRepository {
    fn user_directories() -> Vec<Place> {
        let mut seen: Vec<PathBuf> = Vec::new();
        USER_DIRECTORIES
            .iter()
            .filter_map(|(kind, resolve)| resolve().map(|path| (*kind, path)))
            .filter(|(_, path)| path.is_dir())
            .filter(|(_, path)| {
                let fresh = !seen.contains(path);
                seen.push(path.clone());
                fresh
            })
            .map(|(kind, path)| Place {
                kind,
                section: PlaceSection::Places,
                target: PlaceTarget::Directory(path),
            })
            .collect()
    }
}

impl PlacesRepository for XdgPlacesRepository {
    fn places(&self) -> Vec<Place> {
        let overview = Place {
            kind: PlaceKind::Overview,
            section: PlaceSection::Places,
            target: PlaceTarget::Overview,
        };
        let system = Place {
            kind: PlaceKind::System,
            section: PlaceSection::Devices,
            target: PlaceTarget::Directory(PathBuf::from(FILESYSTEM_ROOT)),
        };
        let trash = Place {
            kind: PlaceKind::Trash,
            section: PlaceSection::Devices,
            target: PlaceTarget::Trash,
        };
        let mut places = vec![overview];
        places.extend(Self::user_directories());
        places.extend([system, trash]);
        places
    }

    fn home(&self) -> Option<PathBuf> {
        dirs::home_dir()
    }
}

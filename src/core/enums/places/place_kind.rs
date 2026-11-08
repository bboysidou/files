use crate::core::enums::assets::icon::Icon;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PlaceKind {
    Overview,
    Home,
    Desktop,
    Documents,
    Downloads,
    Pictures,
    Music,
    Videos,
    System,
    Trash,
}

impl PlaceKind {
    pub fn icon(self) -> Icon {
        match self {
            PlaceKind::Overview => Icon::Overview,
            PlaceKind::Home => Icon::Home,
            PlaceKind::Desktop => Icon::Desktop,
            PlaceKind::Documents => Icon::Document,
            PlaceKind::Downloads => Icon::Download,
            PlaceKind::Pictures => Icon::Image,
            PlaceKind::Music => Icon::Music,
            PlaceKind::Videos => Icon::Video,
            PlaceKind::System => Icon::Drive,
            PlaceKind::Trash => Icon::Trash,
        }
    }

    pub fn label_key(self) -> &'static str {
        match self {
            PlaceKind::Overview => "places.overview",
            PlaceKind::Home => "places.home",
            PlaceKind::Desktop => "places.desktop",
            PlaceKind::Documents => "places.documents",
            PlaceKind::Downloads => "places.downloads",
            PlaceKind::Pictures => "places.pictures",
            PlaceKind::Music => "places.music",
            PlaceKind::Videos => "places.videos",
            PlaceKind::System => "places.system",
            PlaceKind::Trash => "places.trash",
        }
    }
}

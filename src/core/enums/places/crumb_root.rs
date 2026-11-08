use std::borrow::Cow;

use rust_i18n::t;

use crate::core::constants::paths::{HOME_SHORTHAND, PATH_SEPARATOR};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CrumbRoot {
    Home,
    Filesystem,
    Overview,
    Trash,
    Network,
}

impl CrumbRoot {
    pub fn text(self) -> Cow<'static, str> {
        match self {
            CrumbRoot::Home => Cow::Borrowed(HOME_SHORTHAND),
            CrumbRoot::Filesystem => Cow::Borrowed(PATH_SEPARATOR),
            CrumbRoot::Overview => t!("places.overview"),
            CrumbRoot::Trash => t!("places.trash"),
            CrumbRoot::Network => t!("rail.section.network"),
        }
    }
}

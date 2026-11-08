use std::sync::Arc;

use crate::features::tags::data::repositories::xattr_tag_store::XattrTagStore;
use crate::features::tags::domain::repositories::tag_store::TagStore;

pub fn tag_store() -> Arc<dyn TagStore> {
    Arc::new(XattrTagStore)
}

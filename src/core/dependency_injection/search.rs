use std::sync::Arc;

use crate::features::search::data::repositories::local_content_searcher::LocalContentSearcher;
use crate::features::search::data::repositories::walk_search_indexer::WalkSearchIndexer;
use crate::features::search::domain::repositories::content_searcher::ContentSearcher;
use crate::features::search::domain::repositories::search_indexer::SearchIndexer;

pub fn search_indexer() -> Arc<dyn SearchIndexer> {
    Arc::new(WalkSearchIndexer)
}

pub fn content_searcher() -> Arc<dyn ContentSearcher> {
    Arc::new(LocalContentSearcher)
}

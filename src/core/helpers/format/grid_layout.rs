use crate::core::theme::metrics::{
    TILE_GAP, TILE_INNER_GAP, TILE_MIN_WIDTH, TILE_NAME_HEIGHT, TILE_PAD, TILE_THUMB_RATIO,
};

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct GridLayout {
    pub columns: usize,
    pub tile_width: f32,
    pub thumb_height: f32,
}

pub fn grid_layout(available_width: f32) -> GridLayout {
    let columns =
        (((available_width + TILE_GAP) / (TILE_MIN_WIDTH + TILE_GAP)).floor() as usize).max(1);
    let tile_width = ((available_width - TILE_GAP * (columns as f32 - 1.0)) / columns as f32)
        .max(TILE_MIN_WIDTH);
    let thumb_height = (tile_width - TILE_PAD * 2.0) * TILE_THUMB_RATIO;
    GridLayout {
        columns,
        tile_width,
        thumb_height,
    }
}

impl GridLayout {
    pub fn row_height(&self) -> f32 {
        self.thumb_height + TILE_PAD * 2.0 + TILE_INNER_GAP + TILE_NAME_HEIGHT + TILE_GAP
    }
}

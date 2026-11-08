use std::time::Duration;

use gpui::{Context, point, px};

use crate::core::constants::files::BAND_SCROLL_TICK_MS;
use crate::core::theme::metrics::{BAND_SCROLL_EDGE, BAND_SCROLL_GAIN, BAND_SCROLL_MAX_STEP};
use crate::features::browser::presentation::pane_view::PaneView;

impl PaneView {
    pub(super) fn follow_band_edge(&mut self, cx: &mut Context<Self>) {
        let speed = self.band_edge_speed();
        let Some(band) = self.band.as_mut() else {
            return;
        };
        if band.ticker.is_some() || speed == 0.0 {
            return;
        }
        band.ticker = Some(cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor()
                    .timer(Duration::from_millis(BAND_SCROLL_TICK_MS))
                    .await;
                let going = this
                    .update(cx, |this, cx| this.step_band_scroll(cx))
                    .unwrap_or(false);
                if !going {
                    break;
                }
            }
            let _ = this.update(cx, |this, _| {
                if let Some(band) = this.band.as_mut() {
                    band.ticker = None;
                }
            });
        }));
    }

    fn band_edge_speed(&self) -> f32 {
        let Some(band) = self.band.as_ref() else {
            return 0.0;
        };
        let (bounds, _, _) = self.viewport();
        let above = (bounds.top() + px(BAND_SCROLL_EDGE) - band.pointer.y) / px(1.0);
        let below = (band.pointer.y - (bounds.bottom() - px(BAND_SCROLL_EDGE))) / px(1.0);
        let step = |depth: f32| (depth * BAND_SCROLL_GAIN).min(BAND_SCROLL_MAX_STEP);
        if above > 0.0 {
            -step(above)
        } else if below > 0.0 {
            step(below)
        } else {
            0.0
        }
    }

    fn step_band_scroll(&mut self, cx: &mut Context<Self>) -> bool {
        let speed = self.band_edge_speed();
        if speed == 0.0 {
            return false;
        }
        let handle = self.scroll.0.borrow().base_handle.clone();
        let (offset, max) = (handle.offset(), handle.max_offset());
        let target = (offset.y - px(speed)).clamp(-max.y, px(0.0));
        if target == offset.y {
            return true;
        }
        handle.set_offset(point(offset.x, target));
        self.update_band(cx);
        true
    }
}

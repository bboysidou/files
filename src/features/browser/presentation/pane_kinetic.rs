use std::time::{Duration, Instant};

use gpui::{Context, ScrollDelta, ScrollEvent, Window, point, px};

use crate::core::constants::files::{
    BAND_SCROLL_TICK_MS, KINETIC_FRICTION, KINETIC_MIN_SPAN_MS, KINETIC_RELEASE_MS,
    KINETIC_SAMPLE_WINDOW_MS, KINETIC_START_SPEED, KINETIC_STOP_SPEED, MILLIS_PER_SECOND,
};
use crate::features::browser::presentation::pane_view::PaneView;

impl PaneView {
    pub(super) fn track_kinetic(
        &mut self,
        event: &ScrollEvent,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.kinetic.task = None;
        let ScrollDelta::Pixels(delta) = event.delta else {
            self.kinetic.samples.clear();
            return;
        };
        let now = Instant::now();
        self.kinetic
            .samples
            .retain(|(at, _)| now.duration_since(*at).as_millis() < KINETIC_SAMPLE_WINDOW_MS);
        self.kinetic.samples.push((now, delta.y / px(1.0)));
        self.kinetic.task = Some(cx.spawn(async move |this, cx| {
            cx.background_executor()
                .timer(Duration::from_millis(KINETIC_RELEASE_MS))
                .await;
            let Ok(mut speed) = this.update(cx, |this, _| this.release_speed()) else {
                return;
            };
            while speed.abs() >= KINETIC_STOP_SPEED {
                cx.background_executor()
                    .timer(Duration::from_millis(BAND_SCROLL_TICK_MS))
                    .await;
                let Ok(moved) = this.update(cx, |this, cx| this.fling_step(speed, cx)) else {
                    return;
                };
                if !moved {
                    break;
                }
                speed *= KINETIC_FRICTION;
            }
        }));
    }

    pub(super) fn stop_kinetic(&mut self) {
        self.kinetic.task = None;
        self.kinetic.samples.clear();
    }

    fn release_speed(&mut self) -> f32 {
        let samples = std::mem::take(&mut self.kinetic.samples);
        let (Some(first), Some(last)) = (samples.first(), samples.last()) else {
            return 0.0;
        };
        let span = (last.0.duration_since(first.0).as_secs_f32() * MILLIS_PER_SECOND)
            .max(KINETIC_MIN_SPAN_MS);
        let travelled: f32 = samples.iter().map(|(_, delta)| delta).sum();
        let speed = travelled / span * MILLIS_PER_SECOND;
        if speed.abs() < KINETIC_START_SPEED {
            0.0
        } else {
            speed
        }
    }

    fn fling_step(&mut self, speed: f32, cx: &mut Context<Self>) -> bool {
        let handle = self.scroll.0.borrow().base_handle.clone();
        let (offset, max) = (handle.offset(), handle.max_offset());
        let step = speed * BAND_SCROLL_TICK_MS as f32 / MILLIS_PER_SECOND;
        let target = (offset.y + px(step)).clamp(-max.y, px(0.0));
        if target == offset.y {
            return false;
        }
        handle.set_offset(point(offset.x, target));
        cx.notify();
        true
    }
}

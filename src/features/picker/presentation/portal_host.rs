use std::cell::RefCell;
use std::rc::Rc;

use futures::StreamExt;
use futures::channel::mpsc;
use gpui::{
    App, AppContext, AsyncApp, Bounds, Focusable, TitlebarOptions, WindowBounds, WindowOptions, px,
    size,
};

use crate::core::assets::app_icon::app_icon;
use crate::core::constants::app::{PICKER_APP_ID, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::core::dependency_injection::picker::picker_portal;
use crate::features::picker::domain::entities::picker_request::{PickerRequest, PickerResult};
use crate::features::picker::domain::repositories::picker_portal::PickerReply;
use crate::features::picker::presentation::picker_view::PickerView;

const PICKER_SCALE: f32 = 0.8;

fn open_picker(request: PickerRequest, reply: PickerReply, cx: &mut App) {
    let bounds = Bounds::centered(
        None,
        size(
            px(WINDOW_WIDTH * PICKER_SCALE),
            px(WINDOW_HEIGHT * PICKER_SCALE),
        ),
        cx,
    );
    let options = WindowOptions {
        window_bounds: Some(WindowBounds::Windowed(bounds)),
        app_id: Some(PICKER_APP_ID.into()),
        icon: app_icon(),
        titlebar: Some(TitlebarOptions {
            title: Some(request.title.clone().into()),
            ..Default::default()
        }),
        ..Default::default()
    };
    let holder = Rc::new(RefCell::new(Some(reply)));
    let shared = holder.clone();
    let opened = cx.open_window(options, move |window, cx| {
        let reply = shared.borrow_mut().take().expect("reply used once");
        let view = cx.new(|cx| PickerView::new(request, reply, window, cx));
        window.focus(&view.focus_handle(cx), cx);
        view
    });
    if opened.is_err()
        && let Some(reply) = holder.borrow_mut().take()
    {
        let _ = reply.send(PickerResult::Cancelled);
    }
    cx.activate(true);
}

pub fn serve_portal(cx: &mut App) {
    let (sender, mut requests) = mpsc::unbounded();
    let Some(connection) = picker_portal().serve(sender) else {
        eprintln!("files: portal name already taken");
        return cx.quit();
    };
    cx.spawn(async move |cx: &mut AsyncApp| {
        let _connection = connection;
        while let Some((request, reply)) = requests.next().await {
            cx.update(|cx| open_picker(request, reply, cx));
        }
    })
    .detach();
}

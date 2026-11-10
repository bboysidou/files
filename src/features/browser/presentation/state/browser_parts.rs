use std::cell::Cell;
use std::collections::VecDeque;
use std::rc::Rc;

use gpui::{FocusHandle, Pixels, Point, Size, Task, UniformListScrollHandle};

use crate::features::browser::presentation::state::job_state::JobState;
use crate::features::browser::presentation::state::operation_intent::OperationIntent;
use crate::features::browser::presentation::state::preview_state::PreviewState;
use crate::features::browser::presentation::state::toast_model::ToastModel;
use crate::features::operations::domain::entities::file_operation::FileOperation;

#[derive(Default)]
pub struct Toasts {
    pub current: Option<ToastModel>,
    pub generation: usize,
    pub task: Option<Task<()>>,
}

#[derive(Default)]
pub struct Jobs {
    pub operation_task: Option<Task<()>>,
    pub paste_task: Option<Task<()>>,
    pub archive_task: Option<Task<()>>,
    pub job: Option<JobState>,
    pub job_ticker: Option<Task<()>>,
    pub queue: VecDeque<(FileOperation, OperationIntent)>,
}

pub struct QuickLookSlot {
    pub state: Option<PreviewState>,
    pub focus: FocusHandle,
    pub task: Option<Task<()>>,
    pub generation: usize,
    pub scroll: UniformListScrollHandle,
    pub pan_from: Option<Point<Pixels>>,
    pub stage: Rc<Cell<Size<Pixels>>>,
}

impl QuickLookSlot {
    pub fn new(focus: FocusHandle) -> Self {
        Self {
            state: None,
            focus,
            task: None,
            generation: 0,
            scroll: UniformListScrollHandle::new(),
            pan_from: None,
            stage: Rc::default(),
        }
    }
}

pub struct ColumnSlot {
    pub open: bool,
    pub state: Option<PreviewState>,
    pub task: Option<Task<()>>,
    pub generation: usize,
    pub scroll: UniformListScrollHandle,
}

impl ColumnSlot {
    pub fn new(open: bool) -> Self {
        Self {
            open,
            state: None,
            task: None,
            generation: 0,
            scroll: UniformListScrollHandle::new(),
        }
    }
}

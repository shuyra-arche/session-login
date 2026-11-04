use env::{Mark, TaskId};

pub(super) const MARK: Mark = Mark::of("login/system-context");

#[derive(Clone, Copy, env::Frame)]
#[frame(len = 8)]
pub(super) struct Context {
    pub(super) system: TaskId,
}

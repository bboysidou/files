#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum CollisionPolicy {
    KeepBoth,
    Skip,
    Replace,
}

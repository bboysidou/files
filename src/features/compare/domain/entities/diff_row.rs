use crate::core::enums::compare::diff_kind::DiffKind;

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DiffRow {
    pub kind: DiffKind,
    pub left: Option<(usize, String)>,
    pub right: Option<(usize, String)>,
}

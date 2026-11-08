#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ExifFact {
    pub label_key: &'static str,
    pub value: String,
}

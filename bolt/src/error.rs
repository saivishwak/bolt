#[derive(Debug)]
pub enum BoltErrorKind {
    GENERIC,
    INTERNAL,
    EOF,
}

#[derive(Debug)]
pub struct BoltError {
    pub message: String,
    pub kind: BoltErrorKind,
}

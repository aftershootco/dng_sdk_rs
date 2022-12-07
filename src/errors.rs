#[derive(Debug, Clone)]
pub struct Error {
    kind: ErrorKind,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.kind)
    }
}

impl Error {
    pub const fn new(kind: ErrorKind) -> Self {
        Self { kind }
    }
    pub const fn kind(&self) -> ErrorKind {
        self.kind
    }
}

#[derive(Debug, Clone, thiserror::Error, Copy)]
pub enum ErrorKind {
    #[error("Camera has more than 3 matrices :({0})")]
    InvalidColorSpec(usize),
    #[error("Unable to get color temperature")]
    InvalidTemperature,
    #[error("Forward matrices not present")]
    MissingForwardMatrices,
    #[error("Reduction matrices not present")]
    MissingReductionMatrices,
}

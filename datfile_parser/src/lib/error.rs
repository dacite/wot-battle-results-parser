#[derive(thiserror::Error, Debug)]
pub struct NomErrorWrapper {
    pub kind:  nom::error::ErrorKind,
    backtrace: Vec<nom::error::ErrorKind>,
    _input:    Vec<u8>,
}

impl nom::error::ParseError<&[u8]> for NomErrorWrapper {
    fn from_error_kind(input: &[u8], kind: nom::error::ErrorKind) -> Self {
        Self {
            kind,
            backtrace: Vec::new(),
            _input: input.to_vec(),
        }
    }

    fn append(input: &[u8], kind: nom::error::ErrorKind, mut other: Self) -> Self {
        other.backtrace.push(Self::from_error_kind(input, kind).kind);

        other
    }
}

impl std::fmt::Display for NomErrorWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Nom error: {} error", self.kind.description())
    }
}


#[derive(thiserror::Error, Debug)]
pub enum DatfileParseError {
    #[error("{0} has unknown checksum: {1}")]
    UnknownChecksum(String, i32),
}

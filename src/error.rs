use std::error::Error;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub enum Erc721Error {
    RpcError(String),
    NotFound(String),
    Unexpected(String),
}

impl Display for Erc721Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Erc721Error::RpcError(ref desc) => write!(f, "RPC Error: {}", desc),
            Erc721Error::NotFound(ref desc) => write!(f, "Not Found: {}", desc),
            Erc721Error::Unexpected(ref desc) => write!(f, "Unexpected Error: {}", desc),
        }
    }
}

impl Error for Erc721Error {}

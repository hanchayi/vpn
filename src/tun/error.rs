use thiserror::Error;


#[derive(Error, Debug)]
pub enum Error {
    #[error("name too long")]
    NameTooLong,

    #[error("invalid name")]
    InvalidName,

    #[error("unsupported layer")]
    UnsupportedLayer,

    #[error("invalid queues number")]
    InvalidQueuesNumber,

    #[error("invalid descriptor")]
    InvalidDescriptor,

    #[error("invalid address")]
    InvalidAddress,
}

pub type Result<T> = ::std::result::Result<T, Error>;
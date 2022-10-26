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

    #[error("create system socket error")]
    CreateSystemSocketError,

    #[error("io control config error")]
    IOControlConfigError,

    #[error("socket connect error")]
    SocketConnectError,

    #[error("get socket opt error")]
    GetSocketOptError,

    #[error("create internet socket error")]
    CreateInternetSocketError,

    #[error("set address error")]
    SetAddressError,

    #[error("set destination error")]
    SetDestinationError,

    #[error("set broadcast error")]
    SetBroadcastError,
    
    #[error("set netmask error")]
    SetNetmaskError,

    #[error("set mut error")]
    SetMutError,

    #[error("siocgifflags error")]
    SiocgifflagsError,

    #[error("siocsifflags error")]
    SiocsifflagsError,
    
   
}

pub type Result<T> = ::std::result::Result<T, Error>;
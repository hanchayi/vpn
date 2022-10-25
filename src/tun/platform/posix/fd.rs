use std::os::unix::io::{RawFd};
use std::io::{self, Read, Write};
use crate::tun::error::*;
use libc::{self, fcntl, F_GETFL, F_SETFL, O_NONBLOCK};

pub struct Fd(pub RawFd);

impl Fd {
    pub fn new(value: RawFd) -> Result<Self> {
        if value < 0 {
            return Err(Error::InvalidDescriptor)
        }

        Ok(Fd(value))
    }

     /// Enable non-blocking mode
     pub fn set_nonblock(&self) -> io::Result<()> {
        match unsafe { fcntl(self.0, F_SETFL, fcntl(self.0, F_GETFL) | O_NONBLOCK) } {
            0 => Ok(()),
            _ => Err(io::Error::last_os_error()),
        }
    }
}
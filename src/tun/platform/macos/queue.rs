use crate::tun::platform::posix::Fd;
use std::os::unix::io::{AsRawFd, IntoRawFd, RawFd};
use std::io::{self, Read, Write};

pub struct Queue {
    pub tun: Fd,
}

impl Queue {
    pub fn has_packet_information(&self) -> bool {
        // on Android this is always the case
        false
    }

    pub fn set_nonblock(&self) -> io::Result<()> {
        self.tun.set_nonblock()
    }
}

impl AsRawFd for Queue {
    fn as_raw_fd(&self) -> RawFd {
        self.tun.as_raw_fd()
    }
}

impl IntoRawFd for Queue {
    fn into_raw_fd(self) -> RawFd {
        self.tun.into_raw_fd()
    }
}

impl Read for Queue {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.tun.read(buf)
    }

    fn read_vectored(&mut self, bufs: &mut [io::IoSliceMut<'_>]) -> io::Result<usize> {
        self.tun.read_vectored(bufs)
    }
}

impl Write for Queue {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.tun.write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.tun.flush()
    }

    fn write_vectored(&mut self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize> {
        self.tun.write_vectored(bufs)
    }
}
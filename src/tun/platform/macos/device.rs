use libc::{c_char, c_uint, c_void, sockaddr, socklen_t, AF_INET, SOCK_DGRAM, SYSPROTO_CONTROL};

use std::ffi::CStr;
use crate::tun::configuration::{Configuration, Layer};
use crate::tun::error::*;
use crate::tun::platform::macos::sys::*;
use crate::tun::platform::posix::{ Fd, SockAddr };
use crate::tun::device::Device as D;
use std::mem;
use std::io::{self, Read, Write};
use std::ptr;
use std::os::unix::io::{AsRawFd, IntoRawFd, RawFd};
use std::net::Ipv4Addr;

use crate::tun::platform::macos::queue::Queue;

pub struct Device {
    name: String,
    ctl: Fd,
    queue: Queue,
}

impl Device {
    pub fn new(config: &Configuration) -> Result<Self> {
        println!("config{:?}", config);
        let id: u32 = if let Some(name) = config.name.as_ref() {
            if name.len() > IFNAMSIZ {
                return Err(Error::NameTooLong)
            }

            if (!name.starts_with("utun")) {
                return Err(Error::InvalidName)
            }

            name[4..].parse().unwrap()
        } else {
            0
        };

        println!("id{}", id);

        // Only support layer 3
        if config.layer.filter(|l| *l != Layer::L3).is_some() {
            return Err(Error::UnsupportedLayer)
        }

        // Only support queues length to be 1
        let queues_number = config.queues.unwrap_or(1);
        if (queues_number != 1) {
            return Err(Error::InvalidQueuesNumber)
        }


        let mut device = unsafe {
            // Create socket file descriptor
        
            let create_res = Fd::new(
                libc::socket(PF_SYSTEM, SOCK_DGRAM, SYSPROTO_CONTROL));

            match create_res {
                Ok(tun) => {
                    // Create io control info
                    let mut info = ctl_info {
                        ctl_id: 0,
                        // The control name
                        ctl_name: {
                            let mut buffer = [0; 96];
                            //  UTUN_CONTROL_NAME is a registered kernel control
                            for (i, o) in UTUN_CONTROL_NAME.as_bytes().iter().zip(buffer.iter_mut()) {
                                *o = *i as _; 
                            }
                            buffer
                        }
                    };

                    // Config ioctl: Get ID for kernel control
                    if ctliocginfo(
                        tun.0, 
                        &mut info as *mut _ as *mut _
                    ) < 0 {
                        return Err(Error::IOControlConfigError);
                    }

                    // Init system socket addr
                    let addr = sockaddr_ctl {
                        sc_id: info.ctl_id, // kernel control id
                        sc_len: mem::size_of::<sockaddr_ctl>() as _,
                        sc_family: AF_SYSTEM,
                        ss_sysaddr: AF_SYS_CONTROL,
                        sc_unit: id as c_uint,
                        sc_reserved: [0; 5],
                    };

                    // Socket connect
                    if libc::connect(
                        tun.0,
                        &addr as *const sockaddr_ctl as *const sockaddr,
                        mem::size_of_val(&addr) as socklen_t,
                    ) < 0
                    {
                        return Err(Error::SocketConnectError);
                    }

                    // Socket get opt
                    let mut name = [0u8; 64];
                    let mut name_len: socklen_t = 64;

                    // Get socket opt and save to optval
                    if libc::getsockopt(
                        tun.0, // socket file descriptor
                        SYSPROTO_CONTROL, // system control
                        UTUN_OPT_IFNAME, // opt name
                        &mut name as *mut _ as *mut c_void, // opt val ptr
                        &mut name_len as *mut socklen_t, // opt val length
                    ) < 0
                    {
                        return Err(Error::GetSocketOptError);
                    }

                    // Create ctrl socket for ioctl first param 
                    let ctl_res = Fd::new(libc::socket(AF_INET, SOCK_DGRAM, 0));

                    match ctl_res {
                        Ok(ctl) => {
                            Device {
                                // socket opt name
                                name: CStr::from_ptr(name.as_ptr() as *const c_char)
                                    .to_string_lossy()
                                    .into(),
                                queue: Queue { tun: tun },
                                ctl: ctl,
                            }
                        },
                        Err(_) => {
                            return Err(Error::CreateInternetSocketError)
                        },
                    }
                },
                Err(_) => {
                    return Err(Error::CreateSystemSocketError);
                },
            }
        };

        device.configure(config)?;
        Ok(device)
    }


     /// Prepare a new if request.
     pub unsafe fn request(&self) -> ifreq {
        // use zero init
        let mut req: ifreq = mem::zeroed();
        // memcopy modify name
        ptr::copy_nonoverlapping(
            self.name.as_ptr() as *const c_char,
            req.ifrn.name.as_mut_ptr(),
            self.name.len(),
        );

        req
    }

    /// Set the IPv4 alias of the device.
    pub fn set_alias(&mut self, addr: Ipv4Addr, broadaddr: Ipv4Addr, mask: Ipv4Addr) -> Result<()> {
        unsafe {
            let mut req: ifaliasreq = mem::zeroed();
            ptr::copy_nonoverlapping(
                self.name.as_ptr() as *const c_char,
                req.ifran.as_mut_ptr(),
                self.name.len(),
            );

            req.addr = SockAddr::from(addr).into();
            req.broadaddr = SockAddr::from(broadaddr).into();
            req.mask = SockAddr::from(mask).into();

            if siocaifaddr(self.ctl.as_raw_fd(), &req) < 0 {
                return Err(Error::AddRouteError);
            }

            Ok(())
        }
    }


}


impl Read for Device {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        todo!();
    }

    fn read_vectored(&mut self, bufs: &mut [io::IoSliceMut<'_>]) -> io::Result<usize> {
        todo!();
    }
}

impl Write for Device {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        todo!();
    }

    fn flush(&mut self) -> io::Result<()> {
        todo!();
    }

    fn write_vectored(&mut self, bufs: &[io::IoSlice<'_>]) -> io::Result<usize> {
        todo!();
    }
}

impl AsRawFd for Device {
    fn as_raw_fd(&self) -> RawFd {
        self.queue.as_raw_fd()
    }
}

impl D for Device {


    fn name(&self) -> &str {
        &self.name
    }

    // XXX: Cannot set interface name on Darwin.
    fn set_name(&mut self, name: &str) -> Result<()> {
        Err(Error::InvalidName)
    }

    fn enabled(&mut self, value: bool) -> Result<()> {
        unsafe {
            let mut req = self.request();

            // 
            if siocgifflags(self.ctl.as_raw_fd(), &mut req) < 0 {
                return Err(Error::SiocgifflagsError);
            }

            if value {
                req.ifru.flags |= IFF_UP | IFF_RUNNING;
            } else {
                req.ifru.flags &= !IFF_UP;
            }

            if siocsifflags(self.ctl.as_raw_fd(), &req) < 0 {
                return Err(Error::SiocsifflagsError);
            }

            Ok(())
        }
    }

    fn address(&self) -> Result<std::net::Ipv4Addr> {
        todo!()
    }

    fn set_address(&mut self, value: std::net::Ipv4Addr) -> Result<()> {
        unsafe {
            let mut req = self.request();
            req.ifru.addr = SockAddr::from(value).into();

            if siocsifaddr(self.ctl.as_raw_fd(), &req) < 0 {
                return Err(Error::SetAddressError);
            }

            Ok(())
        }
    }

    fn destination(&self) -> Result<std::net::Ipv4Addr> {
        todo!()
    }

    fn set_destination(&mut self, value: std::net::Ipv4Addr) -> Result<()> {
        unsafe {
            let mut req = self.request();
            req.ifru.dstaddr = SockAddr::from(value).into();

            if siocsifdstaddr(self.ctl.as_raw_fd(), &req) < 0 {
                return Err(Error::SetDestinationError);
            }

            Ok(())
        }
    }

    fn broadcast(&self) -> Result<std::net::Ipv4Addr> {
        todo!()
    }

    fn set_broadcast(&mut self, value: std::net::Ipv4Addr) -> Result<()> {
        unsafe {
            let mut req = self.request();
            req.ifru.broadaddr = SockAddr::from(value).into();

            if siocsifbrdaddr(self.ctl.as_raw_fd(), &req) < 0 {
                return Err(Error::SetBroadcastError);
            }

            Ok(())
        }
    }

    fn netmask(&self) -> Result<std::net::Ipv4Addr> {
        todo!()
    }

    fn set_netmask(&mut self, value: std::net::Ipv4Addr) -> Result<()> {
        unsafe {
            let mut req = self.request();
            req.ifru.addr = SockAddr::from(value).into();

            if siocsifnetmask(self.ctl.as_raw_fd(), &req) < 0 {
                return Err(Error::SetNetmaskError);
            }

            Ok(())
        }
    }

    fn mtu(&self) -> Result<i32> {
        todo!()
    }

    // max transfer unit
    fn set_mtu(&mut self, value: i32) -> Result<()> {
        unsafe {
            let mut req = self.request();
            req.ifru.mtu = value;

            if siocsifmtu(self.ctl.as_raw_fd(), &req) < 0 {
                return Err(Error::SetMutError);
            }

            Ok(())
        }
    }

    fn configure(&mut self, config: &Configuration) -> Result<()> {
        if let Some(ip) = config.address {
            self.set_address(ip)?;
        }

        if let Some(ip) = config.destination {
            self.set_destination(ip)?;
        }

        if let Some(ip) = config.broadcast {
            self.set_broadcast(ip)?;
        }

        if let Some(ip) = config.netmask {
            self.set_netmask(ip)?;
        }

        if let Some(mtu) = config.mtu {
            self.set_mtu(mtu)?;
        }

        if let Some(enabled) = config.enabled {
            self.enabled(enabled)?;
        }

        Ok(())
    }
}



#[cfg(test)]
mod test {
    use crate::tun::configuration::Configuration;
    use super::*;

    #[test]
    fn test_new() {
        let device = Device::new(Configuration::default().name("utun5"));
    }
}
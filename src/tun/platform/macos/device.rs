use libc::{ AF_SYSTEM, SOCK_DGRAM, SYSPROTO_CONTROL };

use crate::tun::configuration::{Configuration, Layer};
use crate::tun::error::*;
use crate::tun::platform::macos::sys::{ IFNAMSIZ, UTUN_CONTROL_NAME, ctl_info, ctliocginfo };
use crate::tun::platform::posix::Fd;
use crate::tun::device::Device as D;
use std::io::{self, Read, Write};

pub struct Device {
    name: String,
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
            let tun = Fd::new(libc::socket(
                AF_SYSTEM, 
                SOCK_DGRAM, 
                SYSPROTO_CONTROL
            ));

            let mut info = ctl_info {
                ctl_id: 0,
                ctl_name: {
                    let mut buffer = [0; 96];
                    for (i, o) in UTUN_CONTROL_NAME.as_bytes().iter().zip(buffer.iter_mut()) {
                        *o = *i as _; 
                    }
                    buffer
                }
            };

            // if ctliocginfo(
            //     tun.0, 
            //     &mut info as *mut _ as *mut _
            // ) < 0 {
            //     return Err(io::Error::last_os_error().into());
            // }

            Device {
                name: "sdf".to_string(),
            }
        };

        // device.configure(config)?;
        Ok(device)
    }

}

// impl D for Device {}



#[cfg(test)]
mod test {
    use crate::tun::configuration::Configuration;
    use super::*;

    #[test]
    fn test_new() {
        let device = Device::new(Configuration::default().name("utun4"));
    }
}
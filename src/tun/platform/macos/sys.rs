
use ioctl::*;
use libc::{ c_int, c_uint, c_char, c_short, c_void, c_ushort};
use libc::sockaddr;
// tn name size
pub const IFNAMSIZ: usize = 16;
pub const PF_SYSTEM: c_int = AF_SYSTEM as c_int;

pub const IFF_UP: c_short = 0x1;
pub const IFF_RUNNING: c_short = 0x40;

pub const UTUN_CONTROL_NAME: &str = "com.apple.net.utun_control";
pub const UTUN_OPT_IFNAME: c_int = 2;

pub const AF_SYSTEM: c_char = 32;
pub const AF_SYS_CONTROL: c_ushort = 2;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sockaddr_ctl {
    pub sc_len: c_char,
    pub sc_family: c_char,
    pub ss_sysaddr: c_ushort,
    pub sc_id: c_uint,
    pub sc_unit: c_uint,
    pub sc_reserved: [c_uint; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ifrn {
    pub name: [c_char; IFNAMSIZ],
}


#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifdevmtu {
    pub current: c_int,
    pub min: c_int,
    pub max: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ifru {
    pub addr: sockaddr,
    pub dstaddr: sockaddr,
    pub broadaddr: sockaddr,

    pub flags: c_short,
    pub metric: c_int,
    pub mtu: c_int,
    pub phys: c_int,
    pub media: c_int,
    pub intval: c_int,
    pub data: *mut c_void,
    pub devmtu: ifdevmtu,
    pub wake_flags: c_uint,
    pub route_refcnt: c_uint,
    pub cap: [c_int; 2],
    pub functional_type: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifreq {
    pub ifrn: ifrn,
    pub ifru: ifru,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctl_info {
    pub ctl_id: c_uint,
    pub ctl_name: [c_char; 96],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ifaliasreq {
    pub ifran: [c_char; IFNAMSIZ],
    pub addr: sockaddr,
    pub broadaddr: sockaddr,
    pub mask: sockaddr,
}

/**
 * ioctl: input output control
 * device
|  type    | serial |direct|  size  |
|----------|--------|------|--------|
| 8 bit    | 8 bit  |2 bit |8~14 bit|
|----------|--------|------|--------|
 */
ioctl!(readwrite ctliocginfo with 'N', 3; ctl_info);

ioctl!(write siocsifflags with 'i', 16; ifreq);
ioctl!(readwrite siocgifflags with 'i', 17; ifreq);

ioctl!(write siocsifaddr with 'i', 12; ifreq);
ioctl!(readwrite siocgifaddr with 'i', 33; ifreq);

ioctl!(write siocsifdstaddr with 'i', 14; ifreq);
ioctl!(readwrite siocgifdstaddr with 'i', 34; ifreq);

ioctl!(write siocsifbrdaddr with 'i', 19; ifreq);
ioctl!(readwrite siocgifbrdaddr with 'i', 35; ifreq);

ioctl!(write siocsifnetmask with 'i', 22; ifreq);
ioctl!(readwrite siocgifnetmask with 'i', 37; ifreq);

ioctl!(write siocsifmtu with 'i', 52; ifreq);
ioctl!(readwrite siocgifmtu with 'i', 51; ifreq);

ioctl!(write siocaifaddr with 'i', 26; ifaliasreq);
ioctl!(write siocdifaddr with 'i', 25; ifreq);


// 用C的数据布局策略
#[repr(C)]
#[derive(Debug)]
pub struct aa {
    pub test: String,
    pub i: c_int,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_repr() {
        println!("test_repr");

        println!("aa: {:?}", aa {
            test: "sdf".to_string(),
            i: 1,
        })
    }
}
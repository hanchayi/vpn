use std::net::Ipv4Addr;

use crate::tun::address::IntoAddress;

// OSI layer
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Layer {
    L2,
    L3,
}

#[derive(Default, Debug)]
pub struct Configuration {
    pub name: Option<String>,
    pub layer: Option<Layer>,
    pub queues: Option<usize>,
    pub address: Option<Ipv4Addr>,
    pub destination: Option<Ipv4Addr>,
    pub broadcast: Option<Ipv4Addr>,
    pub netmask: Option<Ipv4Addr>,
    pub mtu: Option<i32>,
    pub enabled: Option<bool>,
}

impl Configuration {
    // Set the name
    pub fn name<S: AsRef<str>>(&mut self, name: S) -> &mut Self {
        self.name = Some(name
            .as_ref() // Ref of str
            .into() // Go to String
        );
        self
    }

    // Set the OSI layer of the operation
    pub fn layer(&mut self, layer: Layer) -> &mut Self {
        self.layer = Some(layer);
        self
    }

    // Set the number of queues
    pub fn queues(&mut self, queues: usize) -> &mut Self {
        self.queues = Some(queues);
        self
    }

    /// Set the address.
    pub fn address<A: IntoAddress>(&mut self, value: A) -> &mut Self {
        self.address = Some(value.into_address().unwrap());
        self
    }

    /// Set the destination address.
    pub fn destination<A: IntoAddress>(&mut self, value: A) -> &mut Self {
        self.destination = Some(value.into_address().unwrap());
        self
    }

    /// Set the broadcast address.
    pub fn broadcast<A: IntoAddress>(&mut self, value: A) -> &mut Self {
        self.broadcast = Some(value.into_address().unwrap());
        self
    }

    /// Set the netmask.
    pub fn netmask<A: IntoAddress>(&mut self, value: A) -> &mut Self {
        self.netmask = Some(value.into_address().unwrap());
        self
    }

    /// Set the MTU.
    pub fn mtu(&mut self, value: i32) -> &mut Self {
        self.mtu = Some(value);
        self
    }
}


use self::device::Device;
use crate::tun::error::*;

pub mod sys;
mod device;
mod queue;

use crate::tun::configuration::Configuration as C;

#[derive(Copy, Clone, Default, Debug)]
pub struct Configuration {

}

pub fn create(configuration: &C) -> Result<Device> {
    Device::new(&configuration)
}
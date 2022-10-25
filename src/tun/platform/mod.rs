#[cfg(unix)]
pub mod posix;

#[cfg(target_os = "macos")]
pub mod macos;
#[cfg(target_os = "macos")]
pub use self::macos::{ create };

#[cfg(test)] 
mod test {
    use crate::tun::configuration::Configuration;

    #[test]

    fn create() {
        let dev = super::create(
                Configuration::default()
                .name("utun6")
            ).unwrap();
    }
}
    

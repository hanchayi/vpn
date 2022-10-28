## Example

``` rust
use std::io::Read;

extern crate tun;

fn main() {
	// create config
	let mut config = tun::Configuration::default();
	config.address((10, 0, 0, 1))
	       .netmask((255, 255, 255, 0))
	       .up();

	// set platform
	#[cfg(target_os = "linux")]
	config.platform(|config| {
		config.packet_information(true);
	});

	// create tun
	let mut dev = tun::create(&config).unwrap();
	let mut buf = [0; 4096];

	loop {
		// read tun
		let amount = dev.read(&mut buf).unwrap();
		println!("{:?}", &buf[0 .. amount]);
	}
}
```



## Refernces

https://github.com/meh/rust-tun
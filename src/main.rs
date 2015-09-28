
extern crate bladerf;

pub fn main() {
  let devices = unsafe{ 
		let mut devices: *mut bladerf::bladerf_devinfo = std::mem::uninitialized();
		
		let n = bladerf::bladerf_get_device_list(&mut devices) as isize;

		let mut safe_device_list: Vec<bladerf::bladerf_devinfo> = Vec::new();
		
		// Catch bladerf function errors
		if n > 0 {

			// Cast array to slice and create a safe array to return
			let device_slice = std::slice::from_raw_parts(devices, n as usize);

			for i in 0..n {
				let local_device = device_slice[i as usize];
				//Safe if this is a copy, unsafe if it is not?
				safe_device_list.push(local_device);
			}
			bladerf::bladerf_free_device_list(devices);
		}
		safe_device_list
	};

  println!("Discovered {} devices", devices.len());

  for d in devices {
    println!("Device: {} Serial: {:?}", d.instance, d.serial);
  }
  
}
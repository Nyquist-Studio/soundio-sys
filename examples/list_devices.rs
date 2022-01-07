use std::ffi::CStr;
use std::process::exit;

use soundio_sys::*;

fn main() {
    unsafe {
        let soundio = soundio_create();
        if soundio.is_null() {
            println!("out of memory");
            exit(1);
        }

        let error = soundio_connect(soundio);
        if error != 0 {
            println!(
                "error: {}",
                CStr::from_ptr(soundio_strerror(error))
                    .to_str()
                    .unwrap_or_default()
            );
            soundio_destroy(soundio);
            exit(1);
        }

        println!("Listening for audio I/O device updates (press Ctrl+C to exit)");

        (*soundio).on_devices_change = Some(device_change_callback);
        loop {
            soundio_wait_events(soundio);
        }
    }
}

unsafe extern "C" fn device_change_callback(soundio: *mut SoundIo) {
    println!("==============================================================");
    {
        let input_count = soundio_input_device_count(soundio);
        let default_input = soundio_default_input_device_index(soundio);
        for i in 0..input_count {
            let device = soundio_get_input_device(soundio, i);
            let device_name = CStr::from_ptr((*device).name).to_str().unwrap_or_default();
            println!(
                "Input Device: {}{}",
                device_name,
                if i == default_input { " (default)" } else { "" }
            );
            soundio_device_unref(device);
        }
    }
    println!("---------------------------------------------------------------");
    {
        let output_count = soundio_output_device_count(soundio);
        let default_output = soundio_default_output_device_index(soundio);
        for i in 0..output_count {
            let device = soundio_get_output_device(soundio, i);
            let device_name = CStr::from_ptr((*device).name).to_str().unwrap_or_default();
            println!(
                "Output Device: {}{}",
                device_name,
                if i == default_output {
                    " (default)"
                } else {
                    ""
                }
            );
            soundio_device_unref(device);
        }
    }
}

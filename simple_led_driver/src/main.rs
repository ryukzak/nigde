extern crate libc;

use std::env;
use std::process::exit;
use std::ptr;
use std::thread;
use std::time::Duration;
// use std::fs::OpenOptions;

const HPS_TO_FPGA_LW_BASE: libc::c_ulong = 0xFF200000;
const HPS_TO_FPGA_LW_SPAN: libc::size_t = 0x0020000;
const CUSTOM_LEDS_0_BASE: libc::size_t = 0;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!(
            "Please enter only one argument that specifies the number of times to blink the LEDs"
        );
        exit(-1);
    }

    // let blink_times: u32 = args[1].parse().unwrap_or_else(|err| {
    //     eprintln!("Invalid argument: {}", err);
    //     exit(-1);
    // });

    let blink_times: u32 = args[1].parse().unwrap_or(0);

    let devmem_fd = unsafe {
        libc::open(
            "/dev/mem\0".as_ptr() as *const _,
            libc::O_RDWR | libc::O_SYNC,
        )
    };
    let devmem_fd = match devmem_fd {
        -1 => {
            eprintln!("Could not open /dev/mem");
            exit(-1);
        }
        fd => fd,
    };

    // let devmem = OpenOptions::new()
    //         .read(true)
    //         .write(true)
    //         .create(false)
    //         .open("/dev/mem")
    //         .expect("Could not open /dev/mem");

    // asdasda //

    let lw_bridge_map = unsafe {
        libc::mmap(
            ptr::null_mut(),
            HPS_TO_FPGA_LW_SPAN as libc::size_t,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_SHARED,
            devmem_fd,
            HPS_TO_FPGA_LW_BASE as libc::off_t,
        )
    };

    if lw_bridge_map == libc::MAP_FAILED {
        eprintln!("Couldn't create mmap.");
        exit(-2);
    }

    let custom_led_map =
        unsafe { (lw_bridge_map as *mut libc::c_uint).offset(CUSTOM_LEDS_0_BASE as isize) };

    for _ in 0..=blink_times {
        unsafe {
            *custom_led_map = 0xFF;
            thread::sleep(Duration::from_micros(500_000));
            *custom_led_map = 0x00;
            thread::sleep(Duration::from_micros(500_000));
        }
    }

    println!("Done!");

    let result = unsafe { libc::munmap(lw_bridge_map, HPS_TO_FPGA_LW_SPAN as libc::size_t) };
    if result < 0 {
        eprintln!("Could not unmap.");
        exit(-3);
    }

    unsafe {
        libc::close(devmem_fd);
    }
}

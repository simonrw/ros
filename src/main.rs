#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(ros::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use ros::println;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("ROS starting");

    ros::init();

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    ros::test_panic_handler(info)
}

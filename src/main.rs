#![cfg_attr(target_arch = "arm", no_std)]
#![cfg_attr(target_arch = "arm", no_main)]


#[cfg(target_arch = "arm")]
mod main_arm;

#[cfg(target_os = "windows")]
mod main_win;

// All project related imports
mod seatalk;
mod seatalk_00;
mod ship_data_traits;

#[cfg(target_os = "windows")]
fn main() {
    main_win::platform_specific_main();
}

#[cfg(target_arch = "arm")]
#[cortex_m_rt::entry]
fn arm_main() -> ! {
    main_arm::real_arm_main();
}

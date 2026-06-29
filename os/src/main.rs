#![deny(warnings)]
#![no_std]
#![no_main]

macro_rules! linker_symbol_addr {
    ($symbol:path) => {
        ($symbol as *const ()).addr()
    };
}

use core::arch::global_asm;
use log::*;
#[macro_use]
mod console;
mod config;
mod device_tree;
mod lang_items;
mod loader;
mod logging;
mod sbi;
mod sync;
pub mod syscall;
pub mod task;
mod timer;
pub mod trap;

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

/// clear BSS segment
fn clear_bss() {
    unsafe extern "C" {
        safe fn sbss();
        safe fn ebss();
    }
    unsafe {
        core::slice::from_raw_parts_mut(
            linker_symbol_addr!(sbss) as *mut u8,
            linker_symbol_addr!(ebss) - linker_symbol_addr!(sbss),
        )
        .fill(0);
    }
}

#[unsafe(no_mangle)]
pub fn chenix_main(_hart_id: usize, dtb_pa: usize) -> ! {
    clear_bss();
    logging::init();
    device_tree::init(dtb_pa);
    info!("[kernel] Welcome to Chenix!");
    trap::init();
    loader::load_apps();
    trap::enable_timer_interrupt();
    timer::set_next_trigger();
    task::run_first_task();
    panic!("Unreachable in chenix_main!");
}

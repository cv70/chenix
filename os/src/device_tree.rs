//! Device tree helpers.

use core::sync::atomic::{AtomicUsize, Ordering};
use fdt::Fdt;
use log::info;

static CLOCK_FREQ: AtomicUsize = AtomicUsize::new(0);

/// Initializes early device information from the flattened device tree.
pub(crate) fn init(dtb_pa: usize) {
    let fdt = unsafe { Fdt::from_ptr(dtb_pa as *const u32) }.expect("invalid device tree");
    let cpus = fdt.root().cpus();
    let clock_freq = cpus
        .common_timebase_frequency()
        .or_else(|| cpus.iter().next().map(|cpu| cpu.timebase_frequency()))
        .expect("missing timebase-frequency in device tree") as usize;
    CLOCK_FREQ.store(clock_freq, Ordering::Relaxed);
    info!("CLOCK_FREQ from device tree: {}", clock_freq);
}

/// Returns the number of `mtime` ticks per second.
pub(crate) fn clock_freq() -> usize {
    let clock_freq = CLOCK_FREQ.load(Ordering::Relaxed);
    assert!(clock_freq != 0, "device tree is not initialized");
    clock_freq
}
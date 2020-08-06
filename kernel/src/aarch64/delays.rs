/// Wait N CPU cycles (ARM CPU only)
pub fn wait_cycles(n: u32) {
    if n > 0 {
        for _ in 0..n {
            unsafe { asm!("nop;") };
        }
    }
}

/// Wait N microsec (ARM CPU only)
pub fn wait_microsec(n: u32) {
    // get the current counter frequency
    let mut frq: u64;
    unsafe { asm!("mrs {}, cntfrq_el0", lateout(reg) frq) };

    // read the current counter
    let mut t: u64;
    unsafe { asm!("mrs {}, cntpct_el0", lateout(reg) t) };

    t += ((frq / 1000) * n as u64) / 1000;

    let mut r: u64;
    unsafe { asm!("mrs {}, cntpct_el0", lateout(reg) r) };
    while r < t {
        unsafe { asm!("mrs {}, cntpct_el0", lateout(reg) r) };
    }
}

pub fn forever() -> ! {
    loop {
        unsafe { asm!("wfe") };
    }
}

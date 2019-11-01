use cortex_m::register::control;

pub fn generate_task_mask(tasks: &[u32]) -> u32 {
    let mut task_mask: u32 = 0;
    for tid in tasks {
        task_mask |= 1 << *tid;
    }
    task_mask
}

pub fn get_msb(val: u32) -> usize {
    let mut res = 0;
    unsafe {
        asm!("clz $1, $0"
        : "=r"(res)
        : "0"(val)
        );
    }
    res = 32 - res;
    if res > 0 {
        res -= 1;
    }
    return res;
}

pub fn check_priv() -> control::Npriv {
    let ctrl_reg = control::read();
    ctrl_reg.npriv()
}

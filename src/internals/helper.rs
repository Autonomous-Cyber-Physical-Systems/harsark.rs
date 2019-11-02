use cortex_m::register::control;
use cortex_m_semihosting::hprintln;

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

pub fn is_privileged() -> bool {
    let mut val = 9;
    unsafe {
            asm!("mrs $0, CONTROL"
            : "=r"(val)
            : 
        )};
    !((val&1) == 1)
}

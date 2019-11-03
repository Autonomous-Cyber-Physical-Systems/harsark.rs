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

pub fn svc_call() {
    unsafe {
        asm!("svc 1");
    }
}
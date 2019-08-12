use core::cmp::max;

const PI: i8 = -1;
static mut RCB: [i8; 32] = [PI; 32];

static mut top: u8 = 0;
static mut PI_Stack: [u8; 32] = [0; 32];

//static mut

pub fn set_permitted_tasks(id: usize, tasks: &[i8]) {
    for tid in tasks {
        unsafe {
            RCB[id] = max(RCB[id], *tid);
        }
    }
}

//fn stack

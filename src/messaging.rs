//use core::alloc::
use cortex_m::interrupt::free as execute_critical;
use crate::task_manager::{
    get_RT,
    release
};
use crate::semaphores::SCB;

pub type Buffer = &'static [u32];

#[derive(Clone, Copy)]
struct TCB {
    dest_buffer: Buffer,
}

static mut TCB_TABLE: [TCB; 32] = [TCB {
    dest_buffer: &[]
}; 32];

#[derive(Clone, Copy)]
struct MCB {
    receivers: [bool; 32],
    src_buffer: Buffer,
}

static mut MCB_TABLE: [MCB; 32] = [MCB {
    receivers: [false; 32],
    src_buffer: &[9],
}; 32];

static mut MsgSCB_TABLE: [SCB; 32] = [SCB {
    flags: [false; 32],
    tasks: [false; 32],
}; 32];


pub fn broadcast(var: usize) {
    execute_critical(|_| {
//        let tasks = unsafe { MCB_TABLE[var].receivers };
        let mut tasks = [false; 32];
        tasks[2] = true;
        msg_signal_release(var, &tasks);
    })
}

pub fn receive(var: usize) -> Result<Buffer,()> {
    execute_critical(|_| {
    let tcb_table = unsafe {&mut TCB_TABLE};
    let mcb_table = unsafe {&mut MCB_TABLE};
    let rt = get_RT();

    if (msg_test_reset(var)) {
        tcb_table[rt].dest_buffer = mcb_table[var].src_buffer;
        return Ok(tcb_table[rt].dest_buffer);
    }
        Err(())
    })
}

fn msg_signal_release(var:usize, tasks: &[bool]) {
    let scb_table = unsafe { &mut MsgSCB_TABLE };
    for i in 0..32 {
        scb_table[var].flags[i] = tasks[i];
    }
    scb_table[var].tasks[2] = true;
    release(&scb_table[var].tasks);
}

fn msg_test_reset(var:usize) -> bool {
        let scb_table = unsafe { &mut MsgSCB_TABLE };
        let rt = get_RT();
        if scb_table[var].flags[rt] == true {
            scb_table[var].flags[rt] = false;
            return true;
        } else {
            return false;
        }
}

//fn configure_msg(var: u32, tasks: &[usize], )
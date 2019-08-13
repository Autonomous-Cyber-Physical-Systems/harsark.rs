//use core::alloc::
use crate::semaphores::SCB;
use crate::task_manager::{get_RT, release};
use cortex_m::interrupt::free as execute_critical;

pub type Buffer = &'static [u32];

#[derive(Clone, Copy)]
struct TCB {
    dest_buffer: Buffer,
}

static mut TCB_TABLE: [TCB; 32] = [TCB { dest_buffer: &[] }; 32];

#[derive(Clone, Copy)]
struct MCB {
    receivers: u32,
    src_buffer: Buffer,
}

static mut MCB_TABLE: [MCB; 32] = [MCB {
    receivers: 0,
    src_buffer: &[],
}; 32];

static mut MsgSCB_TABLE: [SCB; 32] = [SCB { flags: 0, tasks: 0 }; 32];

pub fn broadcast(var: usize) {
    execute_critical(|_| {
        let tasks = unsafe { MCB_TABLE[var].receivers };
        msg_signal_release(var, &tasks);
    })
}

pub fn receive(var: usize) -> Result<Buffer, ()> {
    execute_critical(|_| {
        let tcb_table = unsafe { &mut TCB_TABLE };
        let mcb_table = unsafe { &mut MCB_TABLE };
        let rt = get_RT();

        if (msg_test_reset(var)) {
            tcb_table[rt].dest_buffer = mcb_table[var].src_buffer;
            return Ok(tcb_table[rt].dest_buffer);
        }
        Err(())
    })
}

fn msg_signal_release(semaphore: usize, tasks_mask: &u32) {
    let scb_table = unsafe { &mut MsgSCB_TABLE };
    scb_table[semaphore].flags |= *tasks_mask;
    release(&scb_table[semaphore].tasks);
}

fn msg_test_reset(semaphore: usize) -> bool {
    let scb_table = unsafe { &mut MsgSCB_TABLE };
    let rt = get_RT() as u32;
    let rt_mask = (1 << rt);
    if scb_table[semaphore].flags & rt_mask == rt_mask {
        scb_table[semaphore].flags &= !rt_mask;
        return true;
    } else {
        return false;
    }
}

pub fn configure_msg(var: usize, tasks: &u32, receivers: &u32, src_msg: Buffer) {
    execute_critical(|_| {
        let mcb_table = unsafe { &mut MCB_TABLE };
        let scb_table = unsafe { &mut MsgSCB_TABLE };

        mcb_table[var].src_buffer = src_msg;
        scb_table[var].tasks |= *tasks;
        mcb_table[var].receivers |= *receivers;
    })
}

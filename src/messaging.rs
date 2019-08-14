//use core::alloc::
use crate::semaphores::SCB;
use crate::task_manager::{get_RT, release};
use crate::errors::KernelError;
use crate::config::{MAX_BUFFER_SIZE, MAX_TASKS, MCB_COUNT};

use cortex_m::interrupt::free as execute_critical;
use cortex_m_semihosting::hprintln;

pub type Buffer = &'static [u32];

#[derive(Clone, Copy)]
struct TCB {
    dest_buffer: [u32; MAX_BUFFER_SIZE],
    msg_size: usize,
}

static mut TCB_TABLE: [TCB; MAX_TASKS] = [TCB { dest_buffer: [0; MAX_TASKS], msg_size: 0 }; MAX_TASKS];

#[derive(Clone, Copy)]
struct MCB {
    receivers: u32,
    src_buffer: Buffer,
}

static mut MCB_TABLE: [MCB; MCB_COUNT] = [MCB {
    receivers: 0,
    src_buffer: &[],
}; MCB_COUNT];

static mut MsgSCB_TABLE: [SCB; MCB_COUNT] = [SCB { flags: 0, tasks: 0 }; MCB_COUNT];

pub fn broadcast(var: usize) -> Result<(), KernelError> {
    execute_critical(|_| {
        let mcb = unsafe { MCB_TABLE.get(var) };
        if let Some(mcb) = mcb {
        copy_all (&mcb.receivers, mcb.src_buffer)?;
        msg_signal_release(var, &mcb.receivers);
            return Ok(());
        }
        return Err(KernelError::NotFound);
    })
}

fn copy_all (tasks_mask: &u32, src_msg: Buffer) -> Result<(), KernelError>{
    let tcb_table = unsafe { &mut TCB_TABLE };
    if MAX_BUFFER_SIZE < src_msg.len() {
        return Err(KernelError::BufferOverflow);
    }
    for tid in 1..MAX_TASKS {
        let tid_mask = (1<<tid);
        if tasks_mask & tid_mask == tid_mask {
            for i in 0..src_msg.len() {
                tcb_table[tid].dest_buffer[i] = src_msg[i];
            }
            tcb_table[tid].msg_size = src_msg.len();
        }
    }
    return Ok(());
}

pub fn receive<'a >(var: usize) -> Option<&'a [u32]> {
    execute_critical(|_| {
        let tcb_table = unsafe { &mut TCB_TABLE };
        let mcb_table = unsafe { &mut MCB_TABLE };
        let rt = get_RT();

        if (msg_test_reset(var)) {
            return Some(&tcb_table[rt].dest_buffer[0..tcb_table[rt].msg_size]);
        }
        return None;
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
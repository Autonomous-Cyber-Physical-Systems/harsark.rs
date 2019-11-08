use crate::config::MAX_TASKS;
use crate::KernelError;
use crate::utils::arch::get_msb;
use cortex_m_semihosting::hprintln;
use crate::types::BooleanVector;
use crate::system::types::TaskId;

#[repr(C)]
pub struct Scheduler {
    pub curr_tid: usize,
    pub is_running: bool,
    pub task_control_blocks: [Option<TaskControlBlock>; MAX_TASKS],
    pub blocked_tasks: BooleanVector,
    pub active_tasks: BooleanVector,
    pub is_preemptive: bool,
    pub started: bool,
}

/// A single thread's state
#[repr(C)]
#[derive(Clone, Copy)]
pub struct TaskControlBlock {
    // fields used in assembly, do not reorder them
    pub sp: usize, // current stack pointer of this thread
}

static mut stack0: [u32; 64] = [0; 64];

impl Scheduler {
    pub const fn new() -> Self {
        Self {
            curr_tid: 0,
            is_running: false,
            task_control_blocks: [None; MAX_TASKS],
            active_tasks: 1,
            blocked_tasks: 0,
            is_preemptive: false,
            started: false,
        }
    }

    /// Initialize the switcher system
    pub fn init(&mut self, is_preemptive: bool) {
        self.is_preemptive = is_preemptive;
        /*
            This is the default task, that just puts the board for a power-save mode
            until any event (interrupt/exception) occurs.
        */
            self.create_task(
                0,
                unsafe { &mut stack0 },
                |_| loop {
                    cortex_m::asm::wfe();
                },
                &0,
            )
            .unwrap();
    }

    // The below section just sets up the timer and starts it.
    pub fn start_kernel(&mut self) {
        self.is_running = true;
    }

    pub fn create_task<T: Sized>(
        &mut self,
        priority: usize,
        stack: &mut [u32],
        handler_fn: fn(&T) -> !,
        param: &T,
    ) -> Result<(), KernelError>
        where T: Sync {
        match self.create_tcb(stack, handler_fn, param) {
            Ok(tcb) => {
                self.insert_tcb(priority, tcb)?;
                return Ok(());
            }
            Err(e) => return Err(e),
        }
    }

    fn create_tcb<T: Sized>(
        &self,
        stack: &mut [u32],
        handler: fn(&T) -> !,
        param: &T,
    ) -> Result<TaskControlBlock, KernelError>
        where T: Sync {
        if stack.len() < 32 {
            return Err(KernelError::StackTooSmall);
        }

        let idx = stack.len() - 1;
        let args: u32 = unsafe { core::intrinsics::transmute(param) };
        let pc: usize = handler as usize;

        stack[idx] = 1 << 24; // xPSR
        stack[idx - 1] = pc as u32; // PC
        stack[idx - 7] = args; // args

        let sp: usize = unsafe { core::intrinsics::transmute(&stack[stack.len() - 16]) };
        let tcb = TaskControlBlock { sp: sp as usize };

        Ok(tcb)
    }

    fn insert_tcb(&mut self, idx: usize, tcb: TaskControlBlock) -> Result<(), KernelError> {
        if idx >= MAX_TASKS {
            return Err(KernelError::NotFound);
        }
        self.task_control_blocks[idx] = Some(tcb);
        return Ok(());
    }

    pub fn block_tasks(&mut self, tasks_mask: BooleanVector) {
        self.blocked_tasks |= tasks_mask;
    }

    pub fn unblock_tasks(&mut self, tasks_mask: BooleanVector) {
        self.blocked_tasks &= !tasks_mask;
    }

    pub fn get_next_tid(&self) -> TaskId {
        let mask = self.active_tasks & !self.blocked_tasks;
        return get_msb(mask) as TaskId;
    }

    pub fn release(&mut self, tasks_mask: BooleanVector) {
        self.active_tasks |= tasks_mask;
    }
}

//! # Task Manager
//! The Definition of Data-structures required for task management.
//!
use crate::config::MAX_TASKS;
use crate::utils::arch::{get_msb, save_context, load_context, wait_for_interrupt};
use crate::KernelError;

#[cfg(feature = "task_monitor")]
use crate::kernel::task_monitor::{clear_deadline, set_deadline};

pub type TaskId = u32;
pub type BooleanVector = u32;

/// Maintains state of all tasks in the Kernel
#[repr(C)]
pub struct Scheduler {
    /// The Task id of the currently running task.
    pub curr_tid: usize,
    /// True if the scheduler has started scheduling tasks on the CPU.
    pub started: bool,
    /// An Array of task control blocks corresponding to each task (created only if task exists).
    pub task_control_blocks: [Option<TaskControlBlock>; MAX_TASKS],
    /// A boolean vector in which, if a bit at a position is true, it implies that the task is active and to be scheduled.
    pub blocked_tasks: BooleanVector,
    /// A boolean vector in which, if a bit at a position is true, it implies that the task is blocked and cannot be scheduled even if it’s active.
    pub active_tasks: BooleanVector,
    /// A variable which decided if the scheduler should preemptively schedule tasks or not.
    pub is_preemptive: bool,
    pub preempt_disable_count: u32,
}

/// A single tasks's state
#[cfg(not(feature="task_monitor"))]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct TaskControlBlock {
    /// Holds a reference to the stack pointer for the task.
    stack_pointer: usize, // current stack pointer of this thread
}

#[cfg(feature="task_monitor")]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct TaskControlBlock {
    /// Holds a reference to the stack pointer for the task.
    stack_pointer: usize, // current stack pointer of this thread
    deadline: u32,
}


impl TaskControlBlock {
    pub fn save_context(&self) {
        save_context(self)
    }
    pub fn load_context(&self) {
        load_context(self)
    }
}

impl Scheduler {
    
    /// Returns a new instance of `Scheduler`
    pub const fn new() -> Self {
        Self {
            curr_tid: 0,
            started: false,
            task_control_blocks: [None; MAX_TASKS],
            active_tasks: 1,
            blocked_tasks: 0,
            is_preemptive: false,
            preempt_disable_count: 0,
        }
    }
    
    #[cfg(feature="task_monitor")]
    pub fn init(&mut self) -> Result<(),KernelError>{
        self.is_preemptive = true;
        
        static mut stack0: [u32; 64] = [0; 64];
        self.create_task(
            0,
            100,
            unsafe { &mut stack0 },
            || loop {
                wait_for_interrupt();
            }
        )
    }

    #[cfg(not(feature="task_monitor"))]
    pub fn init(&mut self) -> Result<(),KernelError>{
        self.is_preemptive = true;
        
        static mut stack0: [u32; 64] = [0; 64];
        self.create_task(
            0,
            unsafe { &mut stack0 },
            || loop {
                wait_for_interrupt();
            }
        )
    }

    /// The program counter for the task is pointer value of the function pointer (`handler_fn`). param is a variable whose reference will be made accessible to the task, and this helps in sharing global state with other tasks. Both these values are stored in a specific index of the stack so that when the context\_switch function loads the stack for this task, the appropriate program counter and argument for that function is loaded.
    /// An important thing to note is that the task’s index in the `task_control_blocks` is the priority of the task. Hence there can be only one task of the mentioned priority. Also, another important thing is that the argument param is of a generic type(`T`).
    ///
    /// The `<T: Sync>` informs the compiler that the type `T` must implement the Sync trait. By implementing the Sync trait, a type becomes safe to be shared across tasks. Hence if a type that doesn’t implement Sync trait (like a mutable integer) is passed as param, then the code won’t compile. Kernel primitives like Message and Resource (which are data race safe) implement the Sync trait; hence, it can be passed as param. In this way, the Kernel makes safety a requirement rather than a choice.
    ///
    /// `handler_fn` is of type `fn(&T) -> !`, which implies it is a function pointer which takes a parameter of Type `&T` and infinitely loops. For more details, look into `spawn!` Macro.
    #[cfg(not(feature="task_monitor"))]
    pub fn create_task(
        &mut self,
        priority: usize,
        stack: &mut [u32],
        handler_fn: fn() -> !,
    ) -> Result<(), KernelError>
    {
        let tcb = self.create_tcb(stack, handler_fn)?;
        self.insert_tcb(priority, tcb)
    }
    
    #[cfg(feature="task_monitor")]
    pub fn create_task(
        &mut self,
        priority: usize,
        deadline: u32,
        stack: &mut [u32],
        handler_fn: fn() -> !,
    ) -> Result<(), KernelError>
    {
        let tcb = self.create_tcb(deadline, stack, handler_fn)?;
        self.insert_tcb(priority, tcb)
    }

    /// Creates a TCB corresponding to the tasks details passed onto this method.
    #[cfg(not(feature="task_monitor"))]
    fn create_tcb(
        &self,
        stack: &mut [u32],
        handler: fn() -> !,
    ) -> Result<TaskControlBlock, KernelError>
    {
        if stack.len() < 32 {
            return Err(KernelError::StackTooSmall);
        }

        let pos = stack.len() - 1;
        let pc: usize = handler as usize;

        stack[pos] = 1 << 24; // xPSR
        stack[pos - 1] = pc as u32; // PC

        let stack_pointer: usize = unsafe { core::intrinsics::transmute(&stack[stack.len() - 16]) };
        let tcb = TaskControlBlock {
            stack_pointer: stack_pointer as usize 
        };

        Ok(tcb)
    }
    
    #[cfg(feature="task_monitor")]
    fn create_tcb(
        &self,
        deadline: u32,
        stack: &mut [u32],
        handler: fn() -> !,
    ) -> Result<TaskControlBlock, KernelError>
    {
        if stack.len() < 32 {
            return Err(KernelError::StackTooSmall);
        }

        let pos = stack.len() - 1;
        let pc: usize = handler as usize;

        stack[pos] = 1 << 24; // xPSR
        stack[pos - 1] = pc as u32; // PC

        let stack_pointer: usize = unsafe { core::intrinsics::transmute(&stack[stack.len() - 16]) };
        let tcb = TaskControlBlock {
            deadline, 
            stack_pointer: stack_pointer as usize 
        };

        Ok(tcb)
    }

    /// Inserts the `TCB` into `task_control_blocks` at position `id`.
    fn insert_tcb(&mut self, id: usize, tcb: TaskControlBlock) -> Result<(), KernelError> {
        if id >= MAX_TASKS {
            return Err(KernelError::NotFound);
        }
        self.task_control_blocks[id] = Some(tcb);
        return Ok(());
    }

    /// Appends `tasks_mask` onto `blocked_tasks`.
    pub fn block_tasks(&mut self, tasks_mask: BooleanVector) {
        self.blocked_tasks |= tasks_mask;
    }

    /// Removes `tasks_mask` from `blocked_tasks`.
    pub fn unblock_tasks(&mut self, tasks_mask: BooleanVector) {
        self.blocked_tasks &= !tasks_mask;
    }

    /// Returns the TaskId currently high priority task, which is in ready state.
    /// The highest priority is determined by calculating the most significant bit of boolean vector
    /// corresponding to the tasks in the ready state. The tasks in the ready state can be identified
    /// by the boolean and of `active_tasks` and boolean not(`blocked_tasks`).
    pub fn get_next_tid(&self) -> usize {
        let mask = self.active_tasks & !self.blocked_tasks;
        return get_msb(mask).unwrap();
    }

    /// Updates `active_tasks` with `task_mask`.
    pub fn release(&mut self, tasks_mask: BooleanVector) {
        #[cfg(feature = "task_monitor")] {
            for i in 0..32 {
                if (tasks_mask & 1<<i) > 0 {
                    set_deadline(i as TaskId, self.task_control_blocks[i].unwrap().deadline)
                }
            }
        }
        self.active_tasks |= tasks_mask;
    }
}

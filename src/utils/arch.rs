use crate::kernel::tasks::{os_curr_task_id, os_next_task_id, scheduler};
use crate::system::task_manager::TaskControlBlock;

use cortex_m_semihosting::hprintln;
use cortex_m::interrupt::free as execute_critical;
pub fn get_msb(val: u32) -> usize {
    let mut res;
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
    let val: u32 ;
    unsafe {
        asm!("mrs $0, CONTROL"
            : "=r"(val)
            :
        )
    };
    !((val & 1) == 1)
}

pub fn svc_call() {
    unsafe {
        asm!("svc 1");
    }
}

static empty_task: TaskControlBlock = TaskControlBlock { sp: 0 };

pub fn pendSV_handler() {
    execute_critical(|cs_token| {

        let curr_tid: usize = *os_curr_task_id.borrow(cs_token).borrow();
        let next_tid: usize = *os_next_task_id.borrow(cs_token).borrow();
        let scheduler_inst = scheduler.borrow(cs_token).borrow_mut();
        let next_task = unsafe { scheduler_inst.task_control_blocks[next_tid].as_ref().unwrap() };
        let curr_task = unsafe { scheduler_inst.task_control_blocks[curr_tid].as_ref().unwrap() };
//        let mut curr_task = &empty_task;
//        if unsafe{ scheduler_inst.started } {
//        }
        hprintln!("{} {}", curr_tid, next_tid);


    unsafe {
        asm!(
            "
            /* Disable interrupts: */
	cpsid	i

	/*
	Exception frame saved by the NVIC hardware onto stack:
	+------+
	|      | <- SP before interrupt (orig. SP)
	| xPSR |
	|  PC  |
	|  LR  |
	|  R12 |
	|  R3  |
	|  R2  |
	|  R1  |
	|  R0  | <- SP after entering interrupt (orig. SP + 32 bytes)
	+------+

	Registers saved by the software (PendSV):
	+------+
	|  R7  |
	|  R6  |
	|  R5  |
	|  R4  |
	|  R11 |
	|  R10 |
	|  R9  |
	|  R8  | <- Saved SP (orig. SP + 64 bytes)
	+------+
	*/

	/* Save registers R4-R11 (32 bytes) onto current PSP (process stack
	   pointer) and make the PSP point to the last stacked register (R8):
	   - The MRS/MSR instruction is for loading/saving a special registers.
	   - The STMIA inscruction can only save low registers (R0-R7), it is
	     therefore necesary to copy registers R8-R11 into R4-R7 and call
	     STMIA twice. */
	mrs	r0, psp
	subs	r0, #16
	stmia	r0!,{r4-r7}
	mov	r4, r8
	mov	r5, r9
	mov	r6, r10
	mov	r7, r11
	subs	r0, #32
	stmia	r0!,{r4-r7}
	subs	r0, #16

	/* Save current task's SP: */
	mov	r0, $1
	ldr	r1, [r0]
	str	r0, [r1]

	/* Load next task's SP: */
	mov	r0, $0
	ldr	r0, [r1]

	/* Load registers R4-R11 (32 bytes) from the new PSP and make the PSP
	   point to the end of the exception stack frame. The NVIC hardware
	   will restore remaining registers after returning from exception): */
	ldmia	r0!,{r4-r7}
	mov	r8, r4
	mov	r9, r5
	mov	r10, r6
	mov	r11, r7
	ldmia	r0!,{r4-r7}
	msr	psp, r0

	/* EXC_RETURN - Thread mode with PSP: */
	ldr r0, =0xFFFFFFFD

	/* Enable interrupts: */
	cpsie	i

	bx	r0
            "
            :
            : "r"(next_task), "r"(curr_task)
            : "r0", "r1"
        )
    };
    });
}

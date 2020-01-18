//! # Machine specific
//!
//! Defines functions which are defined majorly in assembly. Thus, might change for one board to another.

use crate::system::scheduler::TaskControlBlock;
/// Returns the MSB of `val`. It is written using CLZ instruction.
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

pub const fn get_msb_const(val: u32) -> usize {
    let mut res = 0;
    let mut i = 0;
    while i < 32 {
        if val & (1<<i) > 0 {
            res = i;
        }
        i += 1;
    }
    return res;
}

/// Creates an SVC Interrupt
pub fn svc_call() {
    unsafe {
        asm!("svc 1");
    }
}

#[inline(always)]
pub fn return_to_psp() {
    unsafe{
        asm!("
        ldr r0, =0xFFFFFFFD
        bx	r0
        ");
    }
}

#[inline(always)]
pub fn save_context(task_stack: &TaskControlBlock) {
    unsafe {
        asm!(
            "
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

    mov	r1, $0
    @bkpt
    @ldr	r1, [r2]
    str	r0, [r1]
            "
            :
            : "r"(task_stack)
            : "r0", "r1"
        )
    };
}

#[inline(always)]
pub fn load_context(task_stack: &TaskControlBlock) {
    unsafe {
        asm!(
            "
            cpsid	i

            mov	r1, $0
            @ldr	r2, =os_next_task
            @ldr	r1, [r2]
            @ldr	r1, [r1]
            ldr	r0, [r1]
            
            ldmia	r0!,{r4-r7}
            mov	r8, r4
            mov	r9, r5
            mov	r10, r6
            mov	r11, r7
            ldmia	r0!,{r4-r7}
            msr	psp, r0
            "
            :
            : "r"(task_stack)
            : "r0", "r1"
        )
    };
}
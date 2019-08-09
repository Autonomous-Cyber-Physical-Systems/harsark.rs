.thumb
.syntax unified

.global __CORTEXM_THREADS_GLOBAL_PTR

.global __CORTEXM_THREADS_wfe
.thumb_func
__CORTEXM_THREADS_wfe:
	wfe
	bx		lr

.global PendSV
.thumb_func
PendSV:
	cpsid	i
	ldr		r1,			=__CORTEXM_THREADS_GLOBAL_PTR /* r1 = &&OS_PTR */
	ldr		r1,			[r1, 0x0] /* r1 = &OS_PTR */
	ldr		r1,			[r1, 0x0] /* r1 = OS_PTR.curr ( &current_thread ) */
	cmp		r1,			0x0
	beq		__CORTEXM_THREADS_PENDSV_RESTORE
	ldr		r2,			[r1, 0x4] /* r2 = current_thread.privileged */
	cmp		r2,			0x0
	beq		__str_unpriv
	mrs		r0,			msp
	b __str_end
	__str_unpriv:
	mrs		r0,			psp
	__str_end:
	stmdb	r0!,		{r4-r11}
	str		r0,			[r1, 0x0] /* current_thread.sp = sp */
	__CORTEXM_THREADS_PENDSV_RESTORE:
	ldr		r1,			=__CORTEXM_THREADS_GLOBAL_PTR	/* r1 = &&OS_PTR */
	ldr		r1,			[r1, 0x0]	/* r1 = &OS_PTR */
	ldr 	r2,			[r1, 0x4]	/* r2 = OS_PTR.next */
	ldr		r3,			[r2, 0x0]	/* r3 = OS_PTR.next.sp */
	ldr		r0,			[r2, 0x4]	/* r0 = OS_PTR.next.privileged */
	ldr		r1,			=__CORTEXM_THREADS_GLOBAL_PTR	/* r1 = &&OS_PTR */
	ldr		r1,			[r1, 0x0]	/* r1 = &OS_PTR */
	ldr		r2,			[r1, 0x4]	/* r2 = &OS.next */
	str		r2,			[r1, 0x0]	/* set OS.curr = os.next */
	ldmia	r3!,		{r4-r11}
	cmp		r0, 		0x0
	beq		__load_unpriv
	movs	r0,			#0x3
	msr		control,	r0
	isb
	msr 	msp,		r3
	ldr 	r0,			=0xFFFFFFF9
	b		__load_end
	__load_unpriv:
	movs	r0,			#0x1
	msr		control,	r0
	isb
	msr 	psp,		r3
	ldr 	r0,			=0xFFFFFFFD
	__load_end:
	cpsie	i
	bx 		r0

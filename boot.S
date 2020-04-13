.section .text.boot, "x"

.global _start

_start:
#if RPI > 1
	/* keep core0 alive */
	mrc p15, #0, r3, c0, c0, #5
	and r3, r3, #3
	cmp r3, #0
	beq 2f
1:
	wfe
	b 1b
2:
#endif

	/* setup the stack. */
	mov sp, #0x8000

	/* relocate to __start */
	mov r3, #0x8000
	ldr r4, =__start
	ldr r9, =__data_end
1:
	ldmia r3!, {r5-r8}
	stmia r4!, {r5-r8}
	cmp r4, r9
	blo 1b

	/* clear out bss. */
	ldr r4, =__bss_start
	ldr r9, =__bss_end
	mov r5, #0
	mov r6, #0
	mov r7, #0
	mov r8, #0
	b       2f
1:
	stmia r4!, {r5-r8}
2:
	cmp r4, r9
	blo 1b

	/* call lib.rs:run() */
	ldr r3, =run
	blx r3


.section .text

.global dummy

dummy:
	mov pc, lr

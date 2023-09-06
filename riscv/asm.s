.option norvc 
	.text
main:
	la sp, _stack_start
	li t0, 1
	li t1, 2
	add t2, t0, t1
	la t0, 0x04000101 //priority 4, enabled, pended, will store at interrupt 0
	la t1, 0x05000101 //priority 5, enabled, pended, will store at interrupt 1
	la a0, 0x1000 //CLIC address
	addi a1, a1, 1
	sw t0, 0(a0) //store at CLIC, interrupt 0
	sw t1, 4(a0) //store at CLIC + 4, interrupt 1
	la t0, 0x50000000
	csrw mtvec, t0 // store vector table address at mtvec
	csrwi 0x347, 8 # raise threshold above interrupt prio 0x347 = mintthresh
	csrwi  mstatus, 8 # t0 = symbol, symbol = x0
	addi sp, sp, 1
	csrwi  0x347, 3 # drop threshold to below prio
	add t0, zero, t0
	addi t0, t0, 1
	addi t0, t0, 1
stop: j stop

.section	.isr0, "ax"
isr0:
	la t0, 0x12345678
	mret
	j stop
.section	.isr1, "ax"
isr1:
	la t0, 0x12345678
	csrwi 0x347, 5
	csrwi mstatus, 8 //will not get preempted by lower prio interrupt
	addi t2, t2, 1
	addi t2, t2, 1
	addi t2, t2, 1
	csrwi 0x347, 4
	addi t2, t2, 1
	csrwi 0x347, 3
	j stop

.data
 isr_0_addr: .word 0x100
 isr_1_addr: .word 0x200

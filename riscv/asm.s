.option norvc 
	.text
init:
	la sp, _stack_start //set stack pointer
main:
	la 	t0, 0x05000101 	//priority 5, enabled, pended, will store at interrupt 0
	la 	t1, 0x04000101 	//priority 4, enabled, pended, will store at interrupt 1
	la 	t2, 0x06000100 	//priority 6, enabled, not pended, will store at interrupt 2
	la 	a0, 0x1000 	//CLIC address
	sw 	t0, 0(a0) 	//store at CLIC, interrupt 0
	sw 	t1, 4(a0) 	//store at CLIC + 4, interrupt 1
	sw 	t2, 8(a0) 	//store at CLIC + 8, interrupt 2
	la 	t0, .vector_table //load vector table address
	csrw 	mtvec, t0 		// store vector table address at mtvec
	csrwi 	0x347, 0 		// set interrupt prio threshold to 0
	csrwi  	mstatus, 8 		//enable global interrupts
	li 	t0, 0
	addi 	t0, t0, 1 		//make sure we return from exception properly, t0 should end up being one by the time we hit stop.
stop: j 	stop 			//loop continue

isr_0:					//this interrupt is pended at the start, it is of middle prio
	li 	a0, 0x1000
	sb 	zero, 0(a0) 	//unpend self
	li	a1, 1
	sb	a1, 8(a0) 	//pend highest prio interrupt 
	li	t0, 0x1337 	//make it obvious we've come here by setting t0 to 1337
	jr 	ra		//return
isr_1:					//this interrupt is pended at the start, it is of low prio
	li 	a0, 0x1000
	sb 	zero, 4(a0) 	//unpend self
	li	t0, 0x1337 	//make it obvious we've come here by setting t1 to 1337
	jr 	ra		//return
isr_2:					//this interrupt is pended from isr_0, it is of highest prio
	li 	a0, 0x1000
	sb 	zero, 8(a0) 	//unpend self
	li	t2, 0x1337 	//make it obvious we've come here by setting t2 to 1337
	jr 	ra		//return

.section	.trap, "ax"
trap_0:
        addi    sp, sp, -0x4c   # allocate space for the context on the stack
        sw      a0, 0x10(sp)    # start by pushing a0 we it to stack CSRs and set threshold
        csrrs   a0, mstatus, x0 # read and stack mstatus 
        sw      a0, 0x00(sp)      
        csrrs   a0, mepc, x0    # read and stack mepc
        sw      a0, 0x04(sp)
        #_STORE_PRIO SUBROUTINE
		csrr	a0, 0x347	# load current threshold
    		sw      a0, 0x08(sp)    # store old threshold on stack
		li     	a0, 0x1000   	# base address for the CLIC MMIO
            	lb 	a0, 3(a0)	# load prio config register of interrupt 0
            	csrw    0x347, a0   	# set the priority
            	csrrsi  x0, mstatus, 8  # enable interrupts (end of critical section)
        #END
        sw      ra, 0x0c(sp)    # stack the caller saved registers
		sw      a1, 0x14(sp)
        sw      a2, 0x18(sp)
        sw      a3, 0x1c(sp)
        sw      a4, 0x20(sp)
        sw      a5, 0x24(sp)
        sw      a6, 0x28(sp)
        sw      a7, 0x2c(sp)
        sw      t0, 0x30(sp)
        sw      t1, 0x34(sp)
        sw      t2, 0x38(sp)
        sw      t3, 0x3c(sp)
        sw      t4, 0x40(sp)
        sw      t5, 0x44(sp)
        sw      t6, 0x48(sp)
        jal     ra, isr_0   # call into the user defined handler

	#RETURN PRIO SUBROUTINE
            lw      a0, 0x08(sp)    	# load the old threshold from the stack
            csrw    0x347, a0   	# set the priority
        #END
    
        lw      a0, 0x00(sp)        # restore CSRs and caller saved registers
        csrrw   x0, mstatus, a0
        lw      a0, 0x04(sp)      
        csrrw   x0, mepc, a0
        lw      ra, 0x0c(sp)
        lw      a0, 0x10(sp)
        lw      a1, 0x14(sp)
        lw      a2, 0x18(sp)
        lw      a3, 0x1c(sp)
        lw      a4, 0x20(sp)
        lw      a5, 0x24(sp)
        lw      a6, 0x28(sp)
        lw      a7, 0x2c(sp)
        lw      t0, 0x30(sp)
        lw      t1, 0x34(sp)
        lw      t2, 0x38(sp)
        lw      t3, 0x3c(sp)
        lw      t4, 0x40(sp)
        lw      t5, 0x44(sp)
        lw      t6, 0x48(sp)
        addi    sp, sp, 0x4c      
        mret                        # return from interrupt

.section	.isr1, "ax"
trap_1:
        addi    sp, sp, -0x4c   # allocate space for the context on the stack
        sw      a0, 0x10(sp)    # start by pushing a0 we it to stack CSRs and set threshold
        csrrs   a0, mstatus, x0 # read and stack mstatus 
        sw      a0, 0x00(sp)      
        csrrs   a0, mepc, x0    # read and stack mepc
        sw      a0, 0x04(sp)
        #_STORE_PRIO SUBROUTINE
		csrr	a0, 0x347	# load current threshold
        	sw      a0, 0x08(sp)    # store old threshold on stack
		li     	a0, 0x1000   	# base address for the CLIC MMIO
            	lb 	a0, 7(a0)	# load prio config register of interrupt 1
            	csrw    0x347, a0   	# set the priority
            	csrrsi  x0, mstatus, 8  # enable interrupts (end of critical section)
        #END
        sw      ra, 0x0c(sp)    # stack the caller saved registers
		sw      a1, 0x14(sp)
        sw      a2, 0x18(sp)
        sw      a3, 0x1c(sp)
        sw      a4, 0x20(sp)
        sw      a5, 0x24(sp)
        sw      a6, 0x28(sp)
        sw      a7, 0x2c(sp)
        sw      t0, 0x30(sp)
        sw      t1, 0x34(sp)
        sw      t2, 0x38(sp)
        sw      t3, 0x3c(sp)
        sw      t4, 0x40(sp)
        sw      t5, 0x44(sp)
        sw      t6, 0x48(sp)
        jal     ra, isr_1   	# call into the user defined handler

	#RETURN PRIO SUBROUTINE
            	lw      a0, 0x08(sp)    # load the old threshold from the stack
		csrw    0x347, a0   	# set the priority
        #END
    
        lw      a0, 0x00(sp)	# restore CSRs and caller saved registers
        csrrw   x0, mstatus, a0
        lw      a0, 0x04(sp)      
        csrrw   x0, mepc, a0
        lw      ra, 0x0c(sp)
        lw      a0, 0x10(sp)
        lw      a1, 0x14(sp)
        lw      a2, 0x18(sp)
        lw      a3, 0x1c(sp)
        lw      a4, 0x20(sp)
        lw      a5, 0x24(sp)
        lw      a6, 0x28(sp)
        lw      a7, 0x2c(sp)
        lw      t0, 0x30(sp)
        lw      t1, 0x34(sp)
        lw      t2, 0x38(sp)
        lw      t3, 0x3c(sp)
        lw      t4, 0x40(sp)
        lw      t5, 0x44(sp)
        lw      t6, 0x48(sp)
        addi    sp, sp, 0x4c      
        mret                        # return from interrupt

trap_2:
        addi    sp, sp, -0x4c   # allocate space for the context on the stack
        sw      a0, 0x10(sp)    # start by pushing a0 we it to stack CSRs and set threshold
        csrrs   a0, mstatus, x0 # read and stack mstatus 
        sw      a0, 0x00(sp)      
        csrrs   a0, mepc, x0    # read and stack mepc
        sw      a0, 0x04(sp)
        #_STORE_PRIO SUBROUTINE
		csrr	a0, 0x347	# load current threshold
            	sw      a0, 0x08(sp)    # store old threshold on stack
		li     	a0, 0x1000   	# base address for the CLIC MMIO
            	lb 	a0, 11(a0)	# load prio config register of interrupt 1
            	csrw    0x347, a0   	# set the priority
            	csrrsi  x0, mstatus, 8  # enable interrupts (end of critical section)
        #END
        sw      ra, 0x0c(sp)    # stack the caller saved registers
		sw      a1, 0x14(sp)
        sw      a2, 0x18(sp)
        sw      a3, 0x1c(sp)
        sw      a4, 0x20(sp)
        sw      a5, 0x24(sp)
        sw      a6, 0x28(sp)
        sw      a7, 0x2c(sp)
        sw      t0, 0x30(sp)
        sw      t1, 0x34(sp)
        sw      t2, 0x38(sp)
        sw      t3, 0x3c(sp)
        sw      t4, 0x40(sp)
        sw      t5, 0x44(sp)
        sw      t6, 0x48(sp)
        jal     ra, isr_2   	# call into the user defined handler

	#RETURN PRIO SUBROUTINE
            lw      a0, 0x08(sp)    	# load the old threshold from the stack
            csrw    0x347, a0   	# set the priority
        #END
    
        lw      a0, 0x00(sp)        # restore CSRs and caller saved registers
        csrrw   x0, mstatus, a0
        lw      a0, 0x04(sp)      
        csrrw   x0, mepc, a0
        lw      ra, 0x0c(sp)
        lw      a0, 0x10(sp)
        lw      a1, 0x14(sp)
        lw      a2, 0x18(sp)
        lw      a3, 0x1c(sp)
        lw      a4, 0x20(sp)
        lw      a5, 0x24(sp)
        lw      a6, 0x28(sp)
        lw      a7, 0x2c(sp)
        lw      t0, 0x30(sp)
        lw      t1, 0x34(sp)
        lw      t2, 0x38(sp)
        lw      t3, 0x3c(sp)
        lw      t4, 0x40(sp)
        lw      t5, 0x44(sp)
        lw      t6, 0x48(sp)
        addi    sp, sp, 0x4c      
        mret                        # return from interrupt

.data
.section .vector_table, "aw"
.word trap_0
.word trap_1
.word trap_2

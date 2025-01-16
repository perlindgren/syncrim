    .option  norvc
    .text
init:
    la       sp, _stack_start    # set stack pointer
    csrwi    0x350, 2            # set stack_depth
    csrsi    0x0, 4              # enable GPIO 2
    csrci    0x1, 4              # GPIO 2 to output
    csrsi    0x5, 4              # GPIO 2 toggle
    csrsi    0x5, 4              # GPIO 2 toggle
    csrsi    0x3, 4              # GPIO 2 set
    csrsi    0x4, 4              # GPIO 2 clear

    csrsi    0x6, 4              # set GPIO 2
    csrsi    0x6, 4              # set GPIO 2 again
    csrsi    0x3, 4              # set GPIO 2 via set reg should do nothing
    csrsi    0x4, 4              # clear GPIO 2 via clear reg
    csrsi    0x6, 4              # set GPIO 2 again
    csrsi    0x5, 4              # toggle GPIO 2 off
    csrsi    0x5, 4              # toggle GPIO 2 on
    csrci    0x6, 4              # clear GPIO 2 
main:
    # setup for individual interrupts done using MMIO
    # we are already at priority 0
    la      t1, 0x01000100      # priority 1, enabled, not pended, will store at interrupt 1
    la      t2, 0x02000100      # priority 2, enabled, not pended, will store at interrupt 2
    la      t3, 0x03000100      # ...
    la      t4, 0x04000100
    la      t5, 0x05000100
    la      t6, 0x06000100
    la      a1, 0x07000100
    la      a2, 0x08000100
    la      a3, 0x09000100
    csrw 0xB01, t1              # configure interrupt 1 via CSR
    csrw 0xB02, t2              # configure interrupt 2 via CSR
    csrw 0xB03, t3
    csrw 0xB04, t4
    csrw 0xB05, t5
    csrw 0xB06, t6
    csrw 0xB07, a1
    csrw 0xB08, a2
  #  csrw 0xB09, a3
    la t1, 0x00005008
    la t2, 50
    #fire interrupt at 50
    sw t2, 0(t1)
    # setup for vector tables and vector table pointers
    la      t0, .clic_vec       # load 0 latency vector table address
    csrw    0x351, t0           # store 0 latency vector table address at super_mtvec
    la      t0, .vector_table   # load vanilla vector table address
    csrw    mtvec, t0           # store vanilla vector table address at mtvec
    csrwi   0x347, 0            # set interrupt prio threshold to 0
    csrwi   mstatus, 8          # enable global interrupts
    li      t1, 0x1             # 1 for pending
    csrrs   zero, 0xB02, t1     # pend interrupt 2, it should now be dispatched
    #jal ra, helper               # enable interrupts via helper
stop:
    ebreak
    j        stop               # finished loop

helper:
   nop
   nop
   csrwi   mstatus, 8          # mock an incoming interrupt in some helper.
   nop
   nop
   jr ra

isr_0:
    j isr_0                     # panic loop, we should never end up here

isr_1: #interrupt 1
    #li a0, 1
    #csrrc zero, 0xB01, a0       # clear pending flag
    jr       ra                 # return

isr_2: #interrupt 2
    li a0, 1
    #csrrc zero, 0xB02, a0       # unpend self
    csrrs zero, 0xB01, a0       # pend interrupt 1
    #nop
    #nop
    #jal ra, helper
    #nop
    #nop
    csrrs zero, 0xB04, a0       # pend interrupt 4
    nop                         # waste a cycle so we can see the return
    jr       ra                 # return
isr_3: #interrupt 3
    li a0, 1
    #csrrc zero, 0xB03, a0       # unpend self
    jr       ra                 # return
isr_4: #interrupt 4
    li a0, 1
    #csrrc zero, 0xB04, a0       # unpend self
    la a1, EXIT_VAR
    lw t0, 0(a1)                # t0 = EXIT_VAR
    bnez t0, isr4_exit          # if EXIT_VAR==0
      sb a0, 0(a1)              # store a 1 at EXIT_VAR
      csrrs zero, 0xB06, a0     # pend interrupt 6
isr4_exit:
    jr       ra                 # return
isr_5: #interrupt 5
    li a0, 1
    #csrrc zero, 0xB05, a0       # unpend self
    jr       ra                 # return
isr_6: #interrupt 6
    li a0, 1
    #csrrc zero, 0xB06, a0       # unpend self
    csrrs zero, 0xB08, a0       # pend interrupt 8
    csrrs zero, 0xB03, a0       # pend interrupt 3
    jr       ra                 # return
isr_7: #interrupt 7
    li a0, 1
    #csrrc zero, 0xB07, a0       # unpend self
    jr       ra                 # return
isr_8: #interrupt 8
    li a0, 1
    #csrrc zero, 0xB08, a0       # unpend self
    csrrs zero, 0xB04, a0       # pend interrupt 4
    csrrs zero, 0xB05, a0       # pend interrupt 5
    jr       ra                 # return
isr_9:
    la t0, 0x5000               # timer base
    lw t1, 0(t0)                # timer lo
    lw t2, 4(t0)                # timer hi
    addi t1, t1, 100            # queue self in 100 cycles
    sw t1, 8(t0)
    sw t2, 12(t0)
    nop
    nop
    jr ra
    .section .trap, "ax"
trap_0:
    addi     sp, sp, -0x4c       # allocate space for the context on the stack
    sw       a0, 0x10(sp)        # start by pushing a0 we it to stack CSRs and set threshold
    csrrs    a0, mstatus, x0     # read and stack mstatus
    sw       a0, 0x00(sp)
    csrrs    a0, mepc, x0        # read and stack mepc
    sw       a0, 0x04(sp)
#_STORE_PRIO SUBROUTINE
    csrr     a0, 0x347           # load current threshold
    sw       a0, 0x08(sp)        # store old threshold on stack
    li       a0, 0x1000          # base address for the CLIC MMIO
    lb       a0, 3(a0)           # load prio config register of interrupt 0
    csrw     0x347, a0           # set the priority
    csrrsi   x0, mstatus, 8      # enable interrupts (end of critical section)
#END
    sw       ra, 0x0c(sp)        # stack the caller saved registers
    sw       a1, 0x14(sp)
    sw       a2, 0x18(sp)
    sw       a3, 0x1c(sp)
    sw       a4, 0x20(sp)
    sw       a5, 0x24(sp)
    sw       a6, 0x28(sp)
    sw       a7, 0x2c(sp)
    sw       t0, 0x30(sp)
    sw       t1, 0x34(sp)
    sw       t2, 0x38(sp)
    sw       t3, 0x3c(sp)
    sw       t4, 0x40(sp)
    sw       t5, 0x44(sp)
    sw       t6, 0x48(sp)

    jal      ra, isr_0           # call into the user defined handler

    lw       a0, 0x00(sp)        # restore CSRs and caller saved registers
    csrrw    x0, mstatus, a0
    lw       a0, 0x04(sp)
    csrrw    x0, mepc, a0
    lw       ra, 0x0c(sp)
    lw       a0, 0x10(sp)
    lw       a1, 0x14(sp)
    lw       a2, 0x18(sp)
    lw       a3, 0x1c(sp)
    lw       a4, 0x20(sp)
    lw       a5, 0x24(sp)
    lw       a6, 0x28(sp)
    lw       a7, 0x2c(sp)
    lw       t0, 0x30(sp)
    lw       t1, 0x34(sp)
    lw       t2, 0x38(sp)
    lw       t3, 0x3c(sp)
    lw       t4, 0x40(sp)
    lw       t5, 0x44(sp)
    #CS
    csrci    mstatus, 0x8     
    lw       t6, 0x08(sp)        # load the old threshold from the stack
    csrw     0x347, t6           # set the priority
    lw       t6, 0x48(sp)
    addi     sp, sp, 0x4c
    mret                         # return from interrupt

trap_1:
    addi     sp, sp, -0x4c       # allocate space for the context on the stack
    sw       a0, 0x10(sp)        # start by pushing a0 we it to stack CSRs and set threshold
    csrrs    a0, mstatus, x0     # read and stack mstatus
    sw       a0, 0x00(sp)
    csrrs    a0, mepc, x0        # read and stack mepc
    sw       a0, 0x04(sp)
#_STORE_PRIO SUBROUTINE
    csrr     a0, 0x347           # load current threshold
    sw       a0, 0x08(sp)        # store old threshold on stack
    li       a0, 0x1000          # base address for the CLIC MMIO
    lb       a0, 7(a0)           # load prio config register of interrupt 1
    csrw     0x347, a0           # set the priority
    csrrsi   x0, mstatus, 8      # enable interrupts (end of critical section)
#END
    sw       ra, 0x0c(sp)        # stack the caller saved registers
    sw       a1, 0x14(sp)
    sw       a2, 0x18(sp)
    sw       a3, 0x1c(sp)
    sw       a4, 0x20(sp)
    sw       a5, 0x24(sp)
    sw       a6, 0x28(sp)
    sw       a7, 0x2c(sp)
    sw       t0, 0x30(sp)
    sw       t1, 0x34(sp)
    sw       t2, 0x38(sp)
    sw       t3, 0x3c(sp)
    sw       t4, 0x40(sp)
    sw       t5, 0x44(sp)
    sw       t6, 0x48(sp)
    jal      ra, isr_1           # call into the user defined handler

    lw       a0, 0x00(sp)        # restore CSRs and caller saved registers
    csrrw    x0, mstatus, a0
    lw       a0, 0x04(sp)
    csrrw    x0, mepc, a0
    lw       ra, 0x0c(sp)
    lw       a0, 0x10(sp)
    lw       a1, 0x14(sp)
    lw       a2, 0x18(sp)
    lw       a3, 0x1c(sp)
    lw       a4, 0x20(sp)
    lw       a5, 0x24(sp)
    lw       a6, 0x28(sp)
    lw       a7, 0x2c(sp)
    lw       t0, 0x30(sp)
    lw       t1, 0x34(sp)
    lw       t2, 0x38(sp)
    lw       t3, 0x3c(sp)
    lw       t4, 0x40(sp)
    lw       t5, 0x44(sp)
    #CS
    csrci    mstatus, 0x8     
    lw       t6, 0x08(sp)        # load the old threshold from the stack
    csrw     0x347, t6           # set the priority
    lw       t6, 0x48(sp)
    addi     sp, sp, 0x4c
    mret                       # return from interrupt

trap_2:
    addi     sp, sp, -0x4c       # allocate space for the context on the stack
    sw       a0, 0x10(sp)        # start by pushing a0 we it to stack CSRs and set threshold
    csrrs    a0, mstatus, x0     # read and stack mstatus
    sw       a0, 0x00(sp)
    csrrs    a0, mepc, x0        # read and stack mepc
    sw       a0, 0x04(sp)
#_STORE_PRIO SUBROUTINE
    csrr     a0, 0x347           # load current threshold
    sw       a0, 0x08(sp)        # store old threshold on stack
    li       a0, 0x1000          # base address for the CLIC MMIO
    lb       a0, 11(a0)          # load prio config register of interrupt 2
    csrw     0x347, a0           # set the priority
    csrrsi   x0, mstatus, 8      # enable interrupts (end of critical section)
#END
    sw       ra, 0x0c(sp)        # stack the caller saved registers
    sw       a1, 0x14(sp)
    sw       a2, 0x18(sp)
    sw       a3, 0x1c(sp)
    sw       a4, 0x20(sp)
    sw       a5, 0x24(sp)
    sw       a6, 0x28(sp)
    sw       a7, 0x2c(sp)
    sw       t0, 0x30(sp)
    sw       t1, 0x34(sp)
    sw       t2, 0x38(sp)
    sw       t3, 0x3c(sp)
    sw       t4, 0x40(sp)
    sw       t5, 0x44(sp)
    sw       t6, 0x48(sp)

    jal      ra, isr_2           # call into the user defined handler

    lw       a0, 0x00(sp)        # restore CSRs and caller saved registers
    csrrw    x0, mstatus, a0
    lw       a0, 0x04(sp)
    csrrw    x0, mepc, a0
    lw       ra, 0x0c(sp)
    lw       a0, 0x10(sp)
    lw       a1, 0x14(sp)
    lw       a2, 0x18(sp)
    lw       a3, 0x1c(sp)
    lw       a4, 0x20(sp)
    lw       a5, 0x24(sp)
    lw       a6, 0x28(sp)
    lw       a7, 0x2c(sp)
    lw       t0, 0x30(sp)
    lw       t1, 0x34(sp)
    lw       t2, 0x38(sp)
    lw       t3, 0x3c(sp)
    lw       t4, 0x40(sp)
    lw       t5, 0x44(sp)
    #CS
    csrci    mstatus, 0x8     
    lw       t6, 0x08(sp)        # load the old threshold from the stack
    csrw     0x347, t6           # set the priority
    lw       t6, 0x48(sp)
    addi     sp, sp, 0x4c
    mret                       # return from interrupt
trap_3:
    addi     sp, sp, -0x4c       # allocate space for the context on the stack
    sw       a0, 0x10(sp)        # start by pushing a0 we it to stack CSRs and set threshold
    csrrs    a0, mstatus, x0     # read and stack mstatus
    sw       a0, 0x00(sp)
    csrrs    a0, mepc, x0        # read and stack mepc
    sw       a0, 0x04(sp)
#_STORE_PRIO SUBROUTINE
    csrr     a0, 0x347           # load current threshold
    sw       a0, 0x08(sp)        # store old threshold on stack
    li       a0, 0x1000          # base address for the CLIC MMIO
    lb       a0, 15(a0)          # load prio config register of interrupt 3
    csrw     0x347, a0           # set the priority
    csrrsi   x0, mstatus, 8      # enable interrupts (end of critical section)
#END
    sw       ra, 0x0c(sp)        # stack the caller saved registers
    sw       a1, 0x14(sp)
    sw       a2, 0x18(sp)
    sw       a3, 0x1c(sp)
    sw       a4, 0x20(sp)
    sw       a5, 0x24(sp)
    sw       a6, 0x28(sp)
    sw       a7, 0x2c(sp)
    sw       t0, 0x30(sp)
    sw       t1, 0x34(sp)
    sw       t2, 0x38(sp)
    sw       t3, 0x3c(sp)
    sw       t4, 0x40(sp)
    sw       t5, 0x44(sp)
    sw       t6, 0x48(sp)

    jal      ra, isr_3           # call into the user defined handler

    lw       a0, 0x00(sp)        # restore CSRs and caller saved registers
    csrrw    x0, mstatus, a0
    lw       a0, 0x04(sp)
    csrrw    x0, mepc, a0
    lw       ra, 0x0c(sp)
    lw       a0, 0x10(sp)
    lw       a1, 0x14(sp)
    lw       a2, 0x18(sp)
    lw       a3, 0x1c(sp)
    lw       a4, 0x20(sp)
    lw       a5, 0x24(sp)
    lw       a6, 0x28(sp)
    lw       a7, 0x2c(sp)
    lw       t0, 0x30(sp)
    lw       t1, 0x34(sp)
    lw       t2, 0x38(sp)
    lw       t3, 0x3c(sp)
    lw       t4, 0x40(sp)
    lw       t5, 0x44(sp)
    #CS
    csrci    mstatus, 0x8     
    lw       t6, 0x08(sp)        # load the old threshold from the stack
    csrw     0x347, t6           # set the priority
    lw       t6, 0x48(sp)
    addi     sp, sp, 0x4c
    mret                       # return from interrupt

trap_4:
    addi     sp, sp, -0x4c       # allocate space for the context on the stack
    sw       a0, 0x10(sp)        # start by pushing a0 we it to stack CSRs and set threshold
    csrrs    a0, mstatus, x0     # read and stack mstatus
    sw       a0, 0x00(sp)
    csrrs    a0, mepc, x0        # read and stack mepc
    sw       a0, 0x04(sp)
#_STORE_PRIO SUBROUTINE
    csrr     a0, 0x347           # load current threshold
    sw       a0, 0x08(sp)        # store old threshold on stack
    li       a0, 0x1000          # base address for the CLIC MMIO
    lb       a0, 19(a0)          # load prio config register of interrupt 4
    csrw     0x347, a0           # set the priority
    csrrsi   x0, mstatus, 8      # enable interrupts (end of critical section)
#END
    sw       ra, 0x0c(sp)        # stack the caller saved registers
    sw       a1, 0x14(sp)
    sw       a2, 0x18(sp)
    sw       a3, 0x1c(sp)
    sw       a4, 0x20(sp)
    sw       a5, 0x24(sp)
    sw       a6, 0x28(sp)
    sw       a7, 0x2c(sp)
    sw       t0, 0x30(sp)
    sw       t1, 0x34(sp)
    sw       t2, 0x38(sp)
    sw       t3, 0x3c(sp)
    sw       t4, 0x40(sp)
    sw       t5, 0x44(sp)
    sw       t6, 0x48(sp)

    jal      ra, isr_4           # call into the user defined handler

    lw       a0, 0x00(sp)        # restore CSRs and caller saved registers
    csrrw    x0, mstatus, a0
    lw       a0, 0x04(sp)
    csrrw    x0, mepc, a0
    lw       ra, 0x0c(sp)
    lw       a0, 0x10(sp)
    lw       a1, 0x14(sp)
    lw       a2, 0x18(sp)
    lw       a3, 0x1c(sp)
    lw       a4, 0x20(sp)
    lw       a5, 0x24(sp)
    lw       a6, 0x28(sp)
    lw       a7, 0x2c(sp)
    lw       t0, 0x30(sp)
    lw       t1, 0x34(sp)
    lw       t2, 0x38(sp)
    lw       t3, 0x3c(sp)
    lw       t4, 0x40(sp)
    lw       t5, 0x44(sp)
    #CS
    csrci    mstatus, 0x8     
    lw       t6, 0x08(sp)        # load the old threshold from the stack
    csrw     0x347, t6           # set the priority
    lw       t6, 0x48(sp)
    addi     sp, sp, 0x4c
    mret                       # return from interrupt
trap_5:
    addi     sp, sp, -0x4c       # allocate space for the context on the stack
    sw       a0, 0x10(sp)        # start by pushing a0 we it to stack CSRs and set threshold
    csrrs    a0, mstatus, x0     # read and stack mstatus
    sw       a0, 0x00(sp)
    csrrs    a0, mepc, x0        # read and stack mepc
    sw       a0, 0x04(sp)
#_STORE_PRIO SUBROUTINE
    csrr     a0, 0x347           # load current threshold
    sw       a0, 0x08(sp)        # store old threshold on stack
    li       a0, 0x1000          # base address for the CLIC MMIO
    lb       a0, 23(a0)          # load prio config register of interrupt 5
    csrw     0x347, a0           # set the priority
    csrrsi   x0, mstatus, 8      # enable interrupts (end of critical section)
#END
    sw       ra, 0x0c(sp)        # stack the caller saved registers
    sw       a1, 0x14(sp)
    sw       a2, 0x18(sp)
    sw       a3, 0x1c(sp)
    sw       a4, 0x20(sp)
    sw       a5, 0x24(sp)
    sw       a6, 0x28(sp)
    sw       a7, 0x2c(sp)
    sw       t0, 0x30(sp)
    sw       t1, 0x34(sp)
    sw       t2, 0x38(sp)
    sw       t3, 0x3c(sp)
    sw       t4, 0x40(sp)
    sw       t5, 0x44(sp)
    sw       t6, 0x48(sp)
    jal      ra, isr_5           # call into the user defined handler

    lw       a0, 0x00(sp)        # restore CSRs and caller saved registers
    csrrw    x0, mstatus, a0
    lw       a0, 0x04(sp)
    csrrw    x0, mepc, a0
    lw       ra, 0x0c(sp)
    lw       a0, 0x10(sp)
    lw       a1, 0x14(sp)
    lw       a2, 0x18(sp)
    lw       a3, 0x1c(sp)
    lw       a4, 0x20(sp)
    lw       a5, 0x24(sp)
    lw       a6, 0x28(sp)
    lw       a7, 0x2c(sp)
    lw       t0, 0x30(sp)
    lw       t1, 0x34(sp)
    lw       t2, 0x38(sp)
    lw       t3, 0x3c(sp)
    lw       t4, 0x40(sp)
    lw       t5, 0x44(sp)
    #CS
    csrci    mstatus, 0x8     
    lw       t6, 0x08(sp)        # load the old threshold from the stack
    csrw     0x347, t6           # set the priority
    lw       t6, 0x48(sp)
    addi     sp, sp, 0x4c
    mret                       # return from interrupt

trap_6:
    addi     sp, sp, -0x4c       # allocate space for the context on the stack
    sw       a0, 0x10(sp)        # start by pushing a0 we it to stack CSRs and set threshold
    csrrs    a0, mstatus, x0     # read and stack mstatus
    sw       a0, 0x00(sp)
    csrrs    a0, mepc, x0        # read and stack mepc
    sw       a0, 0x04(sp)
#_STORE_PRIO SUBROUTINE
    csrr     a0, 0x347           # load current threshold
    sw       a0, 0x08(sp)        # store old threshold on stack
    li       a0, 0x1000          # base address for the CLIC MMIO
    lb       a0, 27(a0)          # load prio config register of interrupt 6
    csrw     0x347, a0           # set the priority
    csrrsi   x0, mstatus, 8      # enable interrupts (end of critical section)
#END
    sw       ra, 0x0c(sp)        # stack the caller saved registers
    sw       a1, 0x14(sp)
    sw       a2, 0x18(sp)
    sw       a3, 0x1c(sp)
    sw       a4, 0x20(sp)
    sw       a5, 0x24(sp)
    sw       a6, 0x28(sp)
    sw       a7, 0x2c(sp)
    sw       t0, 0x30(sp)
    sw       t1, 0x34(sp)
    sw       t2, 0x38(sp)
    sw       t3, 0x3c(sp)
    sw       t4, 0x40(sp)
    sw       t5, 0x44(sp)
    sw       t6, 0x48(sp)

    jal      ra, isr_6           # call into the user defined handler

    lw       a0, 0x00(sp)        # restore CSRs and caller saved registers
    csrrw    x0, mstatus, a0
    lw       a0, 0x04(sp)
    csrrw    x0, mepc, a0
    lw       ra, 0x0c(sp)
    lw       a0, 0x10(sp)
    lw       a1, 0x14(sp)
    lw       a2, 0x18(sp)
    lw       a3, 0x1c(sp)
    lw       a4, 0x20(sp)
    lw       a5, 0x24(sp)
    lw       a6, 0x28(sp)
    lw       a7, 0x2c(sp)
    lw       t0, 0x30(sp)
    lw       t1, 0x34(sp)
    lw       t2, 0x38(sp)
    lw       t3, 0x3c(sp)
    lw       t4, 0x40(sp)
    lw       t5, 0x44(sp)
    #CS
    csrci    mstatus, 0x8     
    lw       t6, 0x08(sp)        # load the old threshold from the stack
    csrw     0x347, t6           # set the priority
    lw       t6, 0x48(sp)
    addi     sp, sp, 0x4c
    mret                       # return from interrupt
trap_7:
    addi     sp, sp, -0x4c       # allocate space for the context on the stack
    sw       a0, 0x10(sp)        # start by pushing a0 we it to stack CSRs and set threshold
    csrrs    a0, mstatus, x0     # read and stack mstatus
    sw       a0, 0x00(sp)
    csrrs    a0, mepc, x0        # read and stack mepc
    sw       a0, 0x04(sp)
#_STORE_PRIO SUBROUTINE
    csrr     a0, 0x347           # load current threshold
    sw       a0, 0x08(sp)        # store old threshold on stack
    li       a0, 0x1000          # base address for the CLIC MMIO
    lb       a0, 31(a0)          # load prio config register of interrupt 7
    csrw     0x347, a0           # set the priority
    csrrsi   x0, mstatus, 8      # enable interrupts (end of critical section)
#END
    sw       ra, 0x0c(sp)        # stack the caller saved registers
    sw       a1, 0x14(sp)
    sw       a2, 0x18(sp)
    sw       a3, 0x1c(sp)
    sw       a4, 0x20(sp)
    sw       a5, 0x24(sp)
    sw       a6, 0x28(sp)
    sw       a7, 0x2c(sp)
    sw       t0, 0x30(sp)
    sw       t1, 0x34(sp)
    sw       t2, 0x38(sp)
    sw       t3, 0x3c(sp)
    sw       t4, 0x40(sp)
    sw       t5, 0x44(sp)
    sw       t6, 0x48(sp)

    jal      ra, isr_7          # call into the user defined handler

    lw       a0, 0x00(sp)        # restore CSRs and caller saved registers
    csrrw    x0, mstatus, a0
    lw       a0, 0x04(sp)
    csrrw    x0, mepc, a0
    lw       ra, 0x0c(sp)
    lw       a0, 0x10(sp)
    lw       a1, 0x14(sp)
    lw       a2, 0x18(sp)
    lw       a3, 0x1c(sp)
    lw       a4, 0x20(sp)
    lw       a5, 0x24(sp)
    lw       a6, 0x28(sp)
    lw       a7, 0x2c(sp)
    lw       t0, 0x30(sp)
    lw       t1, 0x34(sp)
    lw       t2, 0x38(sp)
    lw       t3, 0x3c(sp)
    lw       t4, 0x40(sp)
    lw       t5, 0x44(sp)
    #CS
    csrci    mstatus, 0x8     
    lw       t6, 0x08(sp)        # load the old threshold from the stack
    csrw     0x347, t6           # set the priority
    lw       t6, 0x48(sp)
    addi     sp, sp, 0x4c
    mret                       # return from interrupt
trap_8:
    addi     sp, sp, -0x4c       # allocate space for the context on the stack
    sw       a0, 0x10(sp)        # start by pushing a0 we it to stack CSRs and set threshold
    csrrs    a0, mstatus, x0     # read and stack mstatus
    sw       a0, 0x00(sp)
    csrrs    a0, mepc, x0        # read and stack mepc
    sw       a0, 0x04(sp)
#_STORE_PRIO SUBROUTINE
    csrr     a0, 0x347           # load current threshold
    sw       a0, 0x08(sp)        # store old threshold on stack
    li       a0, 0x1000          # base address for the CLIC MMIO
    lb       a0, 35(a0)          # load prio config register of interrupt 8
    csrw     0x347, a0           # set the priority
    csrrsi   x0, mstatus, 8      # enable interrupts (end of critical section)
#END
    sw       ra, 0x0c(sp)        # stack the caller saved registers
    sw       a1, 0x14(sp)
    sw       a2, 0x18(sp)
    sw       a3, 0x1c(sp)
    sw       a4, 0x20(sp)
    sw       a5, 0x24(sp)
    sw       a6, 0x28(sp)
    sw       a7, 0x2c(sp)
    sw       t0, 0x30(sp)
    sw       t1, 0x34(sp)
    sw       t2, 0x38(sp)
    sw       t3, 0x3c(sp)
    sw       t4, 0x40(sp)
    sw       t5, 0x44(sp)
    sw       t6, 0x48(sp)

    jal      ra, isr_8           # call into the user defined handler

    lw       a0, 0x00(sp)        # restore CSRs and caller saved registers
    csrrw    x0, mstatus, a0
    lw       a0, 0x04(sp)
    csrrw    x0, mepc, a0
    lw       ra, 0x0c(sp)
    lw       a0, 0x10(sp)
    lw       a1, 0x14(sp)
    lw       a2, 0x18(sp)
    lw       a3, 0x1c(sp)
    lw       a4, 0x20(sp)
    lw       a5, 0x24(sp)
    lw       a6, 0x28(sp)
    lw       a7, 0x2c(sp)
    lw       t0, 0x30(sp)
    lw       t1, 0x34(sp)
    lw       t2, 0x38(sp)
    lw       t3, 0x3c(sp)
    lw       t4, 0x40(sp)
    lw       t5, 0x44(sp)
    #CS
    csrci    mstatus, 0x8     
    lw       t6, 0x08(sp)        # load the old threshold from the stack
    csrw     0x347, t6           # set the priority
    lw       t6, 0x48(sp)
    addi     sp, sp, 0x4c
    mret                       # return from interrupt
trap_9:
    addi     sp, sp, -0x4c       # allocate space for the context on the stack
    sw       a0, 0x10(sp)        # start by pushing a0 we it to stack CSRs and set threshold
    csrrs    a0, mstatus, x0     # read and stack mstatus
    sw       a0, 0x00(sp)
    csrrs    a0, mepc, x0        # read and stack mepc
    sw       a0, 0x04(sp)
#_STORE_PRIO SUBROUTINE
    csrr     a0, 0x347           # load current threshold
    sw       a0, 0x08(sp)        # store old threshold on stack
    li       a0, 0x1000          # base address for the CLIC MMIO
    lb       a0, 39(a0)          # load prio config register of interrupt 8
    csrw     0x347, a0           # set the priority
    csrrsi   x0, mstatus, 8      # enable interrupts (end of critical section)
#END
    sw       ra, 0x0c(sp)        # stack the caller saved registers
    sw       a1, 0x14(sp)
    sw       a2, 0x18(sp)
    sw       a3, 0x1c(sp)
    sw       a4, 0x20(sp)
    sw       a5, 0x24(sp)
    sw       a6, 0x28(sp)
    sw       a7, 0x2c(sp)
    sw       t0, 0x30(sp)
    sw       t1, 0x34(sp)
    sw       t2, 0x38(sp)
    sw       t3, 0x3c(sp)
    sw       t4, 0x40(sp)
    sw       t5, 0x44(sp)
    sw       t6, 0x48(sp)

    jal      ra, isr_9           # call into the user defined handler

    lw       a0, 0x00(sp)        # restore CSRs and caller saved registers
    csrrw    x0, mstatus, a0
    lw       a0, 0x04(sp)
    csrrw    x0, mepc, a0
    lw       ra, 0x0c(sp)
    lw       a0, 0x10(sp)
    lw       a1, 0x14(sp)
    lw       a2, 0x18(sp)
    lw       a3, 0x1c(sp)
    lw       a4, 0x20(sp)
    lw       a5, 0x24(sp)
    lw       a6, 0x28(sp)
    lw       a7, 0x2c(sp)
    lw       t0, 0x30(sp)
    lw       t1, 0x34(sp)
    lw       t2, 0x38(sp)
    lw       t3, 0x3c(sp)
    lw       t4, 0x40(sp)
    lw       t5, 0x44(sp)
    #CS
    csrci    mstatus, 0x8     
    lw       t6, 0x08(sp)        # load the old threshold from the stack
    csrw     0x347, t6           # set the priority
    lw       t6, 0x48(sp)
    addi     sp, sp, 0x4c
    mret                       # return from interrupt

    .data
EXIT_VAR: .word 0x00000000
    .section .vector_table, "aw"
    .word    trap_0
    .word    trap_1
    .word    trap_2
    .word    trap_3
    .word    trap_4
    .word    trap_5
    .word    trap_6
    .word    trap_7
    .word    trap_8
    .word    trap_9
    .word    0x20212223
    .word    0x24252627
    .word    0x28292A2B
    .word    0x2C2D2E2F
    .word    0x30313233
    .word    0x34353637
    .word    0x38393A3B
    .word    0x3C3D3E3F
    .word    0x40414243
    .word    0x44454647
    .word    0x48494A4B
    .word    0x4C4D4E4F
    .word    0x50515253
    .word    0x54555657
    .word    0x58595A5B
    .word    0x5C5D5E5F
    .word    0x60616263
    .word    0x64656667
    .word    0x68696A6B
    .word    0x6C6D6E6F
    .word    0x70717273
    .word    0x74757677
    .word    0x78797A7B
    .word    0x7C7D7E7F

    .section .clic_vec, "aw"
    .word    isr_0
    .word    isr_1
    .word    isr_2
    .word    isr_3
    .word    isr_4
    .word    isr_5
    .word    isr_6
    .word    isr_7
    .word    isr_8
    .word    isr_9
    .word    0x20212223
    .word    0x24252627
    .word    0x28292A2B
    .word    0x2C2D2E2F
    .word    0x30313233
    .word    0x34353637
    .word    0x38393A3B
    .word    0x3C3D3E3F
    .word    0x40414243
    .word    0x44454647
    .word    0x48494A4B
    .word    0x4C4D4E4F
    .word    0x50515253
    .word    0x54555657
    .word    0x58595A5B
    .word    0x5C5D5E5F
    .word    0x60616263
    .word    0x64656667
    .word    0x68696A6B
    .word    0x6C6D6E6F
    .word    0x70717273
    .word    0x74757677
    .word    0x78797A7B
    .word    0x7C7D7E7F




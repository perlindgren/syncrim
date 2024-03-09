    .option  norvc
    .text
init:
    la       sp, _stack_start    # set stack pointer
    csrwi    0x350, 2            # set stack_depth
main:
    # setup for individual interrupts done using MMIO
    # we are already at priority 0
    la      t2, 0x02000100      # priority 2, enabled, not pended, will store at interrupt 2
    csrw 0xB02, t2              # configure interrupt 2 via CSR
  #  csrw 0xB09, a3
    # setup for vector tables and vector table pointers
    la t1, 0b1
    csrs 0x0, t1
    nop
    csrc 0x0, t1

    la      t0, .clic_vec       # load 0 latency vector table address
    csrw    0x351, t0           # store 0 latency vector table address at super_mtvec
    la      t0, .vector_table   # load vanilla vector table address
    csrw    mtvec, t0           # store vanilla vector table address at mtvec
    csrwi   0x347, 0            # set interrupt prio threshold to 0
    csrrsi   zero, 0xB02, 0x1     # pend interrupt 2
    jal ra, helper               # enable interrupts via helper
stop:
    j        stop               # finished loop

helper:
   nop
   nop
   csrwi   mstatus, 8          # mock an incoming interrupt in some helper.
   nop
   nop
   jr ra

isr_2: #interrupt 2
    li a0, 1
    csrrc zero, 0xB02, a0       # unpend self
    csrrs zero, 0xB01, a0       # pend interrupt 1
    nop
    nop
    addi sp, sp, -4
    sw ra, 0(sp)
    jal ra, helper              # since interrupt occured at the CSRRW instruction in helper, hitting that instruction now will cause stack_depth to increase erroneously.
    lw ra, 0(sp)
    addi sp, sp, 4
    nop                         # make it obvious that helper call never returns here....
    nop
    jr       ra                 # return

    .section .vector_table, "aw"
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
    .word    0x00000000
    .word    0x00000000
    .word    isr_2
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

    .option  norvc
    .text
init:
    la       sp, _stack_start    # set stack pointer
    csrwi    0x350, 2            # set stack_depth
main:
    csrwi 0x300, 8 # enable global interrupts
    la t1, isr_2   # address to isr_2
    csrw 0xB02, t1 # write above to interrupt 2 vector
    la t1, isr_0   # address to isr_0
    csrw 0xB00, t1 # write above to interrupt 0 vector
    la t1, 0b11010 # prio 0b110, enable 0b1, pend 0b0
    csrw 0xB22, t1 # write above to interrupt 2
    la t1, 0b1     # pend 0b1
    csrs 0xB22, t1 # write above to interrupt 2
    li t2, 0b11110000 # cmp value 0b1111 = 15, interrupt every 15 cycles , prescaler 0b0000
    csrw 0x400, t2 # write above to timer CSR
    la t1, 0b11110 # prio 0b111, enable, 0b1, pend 0b0
    csrw 0xB20, t1 # write above to interrupt 0 (timer interrupt)
stop:
    j stop        # wfi

isr_0: #timer interrupt
    la t0, .toggled # &static mut toggled state
    lw t2, 0(t0)    # deref toggled
    li t1, 1 
    bnez t2, led_off    # if toggled == 1 then led_off else led_on
led_on:
    csrs 0x0, t1    # set bit 0 (t1 = 1) in GPIO CSR (LED on)
    sw t1, 0(t0)    # store 1 at toggled state var
    csrr t3, 0xB42  # load timestamp
    jr ra           # return (if is unrolled to save some cycles)
led_off:
    csrc 0x0, t1    # clear bit 0 (t1 = 1) in GPIO CSR (LED off)
    sw zero, 0(t0)  # store 0 at toggled state var
    csrr t3, 0xB42  # load timestamp
    jr ra           # return (if is unrolled to save some cycles)

isr_2: #interrupt 2
    csrr t3, 0xB42              # read time stamp
    jr       ra                 # return

    .data
.toggled: .word 0x0

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
